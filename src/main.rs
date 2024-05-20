use std::io::StdoutLock;

use anyhow::{Context, Ok};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
    #[serde(rename = "type")]
    ty: String,
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        echo: String,
    },
}

struct EchoNode {
    id: usize,
}

impl EchoNode {
    pub fn step(
        &mut self,
        input: Message,
        output: &mut serde_json::Serializer<StdoutLock>,
    ) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo },
                        ty: todo!(),
                    },
                };
                reply
                    .serialize(output)
                    .context("serialize response to echo")?;
                self.id += 1;
            }
            Payload::EchoOk { .. } => {}
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let stdout = std::io::stdout().lock();

    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let mut output = serde_json::Serializer::new(stdout);
    let mut state = EchoNode { id: 0 };
    for input in inputs {
        let input = input.context("Malestrom input could not be deserliased")?;
        state
            .step(input, &mut output)
            .context("Node step function failed")?;
    }

    Ok(())
}
