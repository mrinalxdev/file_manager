use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dest: String,
    body: Body,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
    #[serde(rename = "type")]
    ty: String,
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,
}

fn main() {
    println!("Hello, world!");
}
