use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpListener;

use crate::xml::response::init::Init;
use crate::{FeatureSet, Status};

pub struct XdebugConnection {
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
    transaction_id: i8,
}

impl XdebugConnection {
    pub async fn initialize() -> Option<XdebugConnection> {
        let listener = TcpListener::bind("0.0.0.0:9003").await.unwrap();

        if let Ok((socket, _)) = listener.accept().await {
            drop(listener);

            let (read_half, write_half) = socket.into_split();
            let reader = BufReader::new(read_half);

            let mut xdebug_connection = Self {
                reader,
                writer: write_half,
                transaction_id: 0,
            };

            xdebug_connection.read_init().await;

            return Some(xdebug_connection);
        }

        None
    }

    pub async fn set_feature(&mut self, name: &str, value: &str) -> FeatureSet {
        self.send_command("feature_set", Some(HashMap::from([("-n", name), ("-v", value)]))).await;

        FeatureSet::from_str(self.read_response().await.as_str())
    }

    pub async fn status(&mut self) -> Status {
        self.send_command("status", None).await;

        Status::from_str(self.read_response().await.as_str())
    }

    async fn send_command(&mut self, command: &str, args: Option<HashMap<&str, &str>>) {
        self.increase_transaction_id();

        let mut command = String::from(command.trim());

        command.push_str(" ");
        command.push_str("-i");
        command.push_str(" ");
        command.push_str(self.transaction_id.to_string().as_str());

        if let Some(args) = args {
            args.iter().for_each(|(&key, &val)| {
                command.push_str(" ");
                command.push_str(key);
                command.push_str(" ");
                command.push_str(val);
            });
        }

        command.push_str("\0");

        self.writer.write_all(command.as_bytes()).await.unwrap()
    }

    async fn read_init(&mut self) -> Init {
        let _xml_size = self.read_until(0x00).await;
        let xml = self.read_until(0x00).await;

        Init::from_str(xml.as_str())
    }

    async fn read_response(&mut self) -> String {
        let _xml_size = self.read_until(0x00).await;
        let xml = self.read_until(0x00).await;

        xml
    }

    fn increase_transaction_id(&mut self) {
        self.transaction_id += 1;
    }

    async fn read_until(&mut self, delimiter: u8) -> String {
        let mut read_until_buffer = Vec::new();

        // get the size of xml to read
        self.reader.read_until(delimiter, &mut read_until_buffer).await.unwrap();

        // // remove null byte in the end
        read_until_buffer.remove(read_until_buffer.len() - 1);

        String::from_utf8(read_until_buffer).unwrap()
    }
}