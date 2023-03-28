//! A library to decode ink! contract JSON into Rust types using Serde.
//!
//! This crate provides a simple way to parse ink! contract JSON strings
//! and deserialize them into Rust types for easy manipulation and interaction.
//!
//! Example:
//!
//! ```rust
//! use ink_contract_decoder::decode_ink_contract;
//!
//! fn main() {
//!     let json_str = r#"
//!     {
//!         // Your JSON contract string here
//!     }
//!     "#;
//!
//!     let ink_contract = decode_ink_contract(json_str).unwrap();
//!     println!("{:#?}", ink_contract);
//! }
//! ```
//!
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct InkContract {
    pub source: Source,
    pub contract: Contract,
    pub spec: Spec,
    pub storage: Storage,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Source {
    pub hash: String,
    pub language: String,
    pub compiler: String,
    #[serde(with = "serde_bytes")]
    pub wasm: ByteBuf,
    pub build_info: BuildInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BuildInfo {
    pub build_mode: String,
    pub cargo_contract_version: String,
    pub rust_toolchain: String,
    pub wasm_opt_settings: WasmOptSettings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WasmOptSettings {
    pub keep_debug_symbols: bool,
    pub optimization_passes: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contract {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Spec {
    pub constructors: Vec<Constructor>,
    pub docs: Vec<String>,
    pub events: Vec<String>,
    pub lang_error: LangError,
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Constructor {
    pub args: Vec<String>,
    pub docs: Vec<String>,
    pub label: String,
    pub payable: bool,
    pub return_type: ReturnType,
    pub selector: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReturnType {
    pub display_name: Vec<String>,
    pub r#type: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LangError {
    pub display_name: Vec<String>,
    pub r#type: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub args: Vec<Arg>,
    pub docs: Vec<String>,
    pub label: String,
    pub mutates: bool,
    pub payable: bool,
    pub return_type: ReturnType,
    pub selector: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Arg {
    pub label: String,
    pub r#type: ReturnType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Storage {
    pub root: Root,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    pub layout: Layout,
    pub root_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Layout {
    pub r#struct: Struct,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Struct {
    pub name: String,
}

/// Decode the JSON string into an `InkContract` struct.
pub fn decode_ink_contract(json_str: &str) -> Result<InkContract, serde_json::Error> {
    serde_json::from_str(json_str)
}
