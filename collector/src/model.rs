use serde::Serialize;

#[derive(Serialize)]
pub struct LogMessage {
    pub message:String,
}