/*
    Appellation: template-rs-cloudflare <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use worker::*;

#[event(fetch)]
pub async fn main(_req: Request, _env: Env, _ctx: worker::Context) -> Result<Response> {
    
    let data = Message {
        hello: String::from("world"),
    
    };
    Response::from_json(&data)
}


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct Message {
    hello: String,
}

impl Message {
    pub fn new(hello: impl ToString) -> Self {
        Self { hello: hello.to_string() }
    }
}