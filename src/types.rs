use serde::Serialize;

#[derive(Serialize)]
pub enum ProtocolType {
    #[serde(rename = "token_20")]
    Token20,
    #[serde(rename = "token_721")]
    Token721,
    #[serde(rename = "token_1155")]
    Token1155,
}