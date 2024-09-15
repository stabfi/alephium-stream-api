use crate::types::events::Event;
use crate::types::Val;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub id: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetState {
    #[serde(rename = "attoAlphAmount")]
    pub atto_alph_amount: String,
    pub tokens: Option<Vec<Token>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractState {
    pub address: String,
    pub bytecode: String,
    #[serde(rename = "codeHash")]
    pub code_hash: String,
    #[serde(rename = "initialStateHash")]
    pub initial_state_hash: Option<String>,
    #[serde(rename = "immFields")]
    pub imm_fields: Vec<Val>,
    #[serde(rename = "mutFields")]
    pub mut_fields: Vec<Val>,
    pub asset: AssetState,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetOutput {
    pub hint: i32,
    pub key: String,
    #[serde(rename = "attoAlphAmount")]
    pub atto_alph_amount: String,
    pub address: String,
    pub tokens: Vec<Token>,
    #[serde(rename = "lockTime")]
    pub lock_time: i64,
    pub message: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractOutput {
    pub hint: i32,
    pub key: String,
    #[serde(rename = "attoAlphAmount")]
    pub atto_alph_amount: String,
    pub address: String,
    pub tokens: Vec<Token>,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DebugMessage {
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputAsset {
    pub address: String,
    pub asset: AssetState,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractCallBody {
    pub group: u8,
    #[serde(rename = "worldStateBlockHash")]
    pub world_state_block_hash: Option<String>,
    #[serde(rename = "txId")]
    pub tx_id: Option<String>,
    pub address: String,
    #[serde(rename = "callerAddress")]
    pub caller_address: Option<String>,
    #[serde(rename = "methodIndex")]
    pub method_index: i32,
    pub args: Option<Vec<Val>>,
    #[serde(rename = "interestedContracts")]
    pub interested_contracts: Option<Vec<String>>,
    #[serde(rename = "inputAssets")]
    pub input_assets: Option<Vec<InputAsset>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Output {
    Asset(AssetOutput),
    Contract(ContractOutput),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ContractCallResponse {
    Success {
        r#type: String,
        returns: Vec<Val>,
        #[serde(rename = "gasUsed")]
        gas_used: u32,
        contracts: Vec<ContractState>,
        #[serde(rename = "txInputs")]
        tx_inputs: Vec<String>,
        #[serde(rename = "txOutputs")]
        tx_outputs: Vec<Output>,
        events: Vec<Event>,
        #[serde(rename = "debugMessages")]
        debug_messages: Vec<DebugMessage>,
    },
    Error {
        error: String,
        r#type: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultipleCallContractBody {
    pub calls: Vec<ContractCallBody>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultipleCallContractResponse {
    pub results: Vec<ContractCallResponse>,
}
