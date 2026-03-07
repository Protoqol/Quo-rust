use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, PartialEq)]
pub struct QuoPayload {
    pub meta: QuoPayloadMeta,
    pub language: QuoPayloadLanguage,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct QuoPayloadVariable {
    pub var_type: String,
    pub name: String,
    pub value: String,
    pub mutable: bool,
    pub is_constant: bool,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct QuoPayloadMeta {
    pub id: u32,
    pub uid: String,
    pub origin: String,
    pub sender_origin: String,
    pub time_epoch_ms: i64,
    pub variable: QuoPayloadVariable,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum QuoPayloadLanguage {
    Rust,
    Php,
    Javascript,
    Typescript,
    Python,
    Ruby,
    Go,
    #[default]
    Unknown,
}
