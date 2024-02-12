use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, ToSocketAddrs};

use crate::xml::response::init::Init;
use crate::{BreakpointSet, ContextGet, Detach, FeatureSet, Run, Status, XcmdGetExecutableLines};
use crate::xml::response::context_names::ContextNames;
use crate::xml::response::stack_get::StackGet;
use crate::xml::response::step_into::StepInto;

pub struct Connection {
    pub reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
    transaction_id: i8,
}

impl Connection {
    pub async fn initialize<A: ToSocketAddrs>(addr: A) -> Option<Connection> {
        let listener = TcpListener::bind(addr).await.unwrap();

        if let Ok((socket, _)) = listener.accept().await {
            drop(listener);

            let (read_half, write_half) = socket.into_split();
            let reader = BufReader::new(read_half);

            let mut connection = Self {
                reader,
                writer: write_half,
                transaction_id: 0,
            };

            connection.read_init().await;

            return Some(connection);
        }

        None
    }

    pub async fn xcmd_get_executable_lines(&mut self, depth: &str) -> XcmdGetExecutableLines {
        self.send_command("xcmd_get_executable_lines", Some(HashMap::from([("-d", depth)]))).await;

        XcmdGetExecutableLines::from_str(self.read_response().await.as_str())
    }

    pub async fn breakpoint_set(&mut self, breakpoint_type: &str, lineno: &str
                                , filename: &str
    ) -> BreakpointSet {
        self.send_command("breakpoint_set", Some(HashMap::from([("-t", breakpoint_type), ("-n", lineno)
            , ("-f", filename)
        ]))).await;

        BreakpointSet::from_str(self.read_response().await.as_str())
    }

    pub async fn feature_set(&mut self, name: &str, value: &str) -> FeatureSet {
        self.send_command("feature_set", Some(HashMap::from([("-n", name), ("-v", value)]))).await;

        FeatureSet::from_str(self.read_response().await.as_str())
    }

    pub async fn status(&mut self) -> Status {
        self.send_command("status", None).await;

        Status::from_str(self.read_response().await.as_str())
    }

    pub async fn step_into(&mut self) -> StepInto {
        self.send_command("step_into", None).await;

        StepInto::from_str(self.read_response().await.as_str())
    }

    pub async fn stack_get(&mut self) -> StackGet {
        self.send_command("stack_get", None).await;

        StackGet::from_str(self.read_response().await.as_str())
    }

    pub async fn run(&mut self) -> Run {
        self.send_command("run", None).await;

        Run::from_str(self.read_response().await.as_str())
    }

    pub async fn detach(&mut self) -> Detach {
        self.send_command("detach", None).await;

        Detach::from_str(self.read_response().await.as_str())
    }

    pub async fn context_names(&mut self) -> ContextNames {
        self.send_command("context_names", None).await;

        ContextNames::from_str(self.read_response().await.as_str())
    }

    pub async fn context_get(&mut self, depth: Option<&str>, context: Option<&str>) -> ContextGet {
        let mut args = HashMap::new();

        if let Some(depth) = depth {
            args.insert("-d", depth);
        }

        if let Some(context) = context {
            args.insert("-c", context);
        }

        self.send_command("context_get", Some(args)).await;

        ContextGet::from_str(self.read_response().await.as_str())
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
        let _xml_size = self.read_until(0x0).await;
        let xml = self.read_until(0x0).await;

        Init::from_str(xml.as_str())
    }

    async fn read_response(&mut self) -> String {
        let _xml_size = self.read_until(0x0).await;
        let xml = self.read_until(0x0).await;

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