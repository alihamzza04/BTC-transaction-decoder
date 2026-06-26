use std::error::Error;
use self::transaction::Transaction;
use crate::transaction::Decodable;
mod transaction;

pub fn decode(transaction_hex: String)-> Result<String, Box<dyn Error>>{
    let transaction_bytes = hex::decode(transaction_hex).map_err(|e| format!("The decode error: {}", e))?;

    let transaction = Transaction::consensus_decode(&mut transaction_bytes.as_slice())?;

    Ok(serde_json::to_string_pretty(&transaction)?)
}