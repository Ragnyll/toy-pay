use serde::Deserialize;
use thiserror::Error;

/// A Transaction that can be processed by a client
pub enum Transaction {
    // Amount has a precision of 4 decimal places
    Withdraw { tx_id: u32, amount: f32 },
    // Amount has a precision of 4 decimal places
    Deposit { tx_id: u32, amount: f32 },
    Dispute { tx_id: u32 },
    Resolve { tx_id: u32 },
    Chargeback { tx_id: u32 },
}

impl Transaction {
    pub fn from_input_transaction(
        itx: &InputTransaction,
    ) -> Result<Self, TransactionConversionError> {
        match itx.tx_type.as_str() {
            "withdrawal" => Ok(Transaction::Withdraw {
                tx_id: itx.tx_id,
                amount: itx.amount.unwrap(),
            }),
            "deposit" => Ok(Transaction::Deposit {
                tx_id: itx.tx_id,
                amount: itx.amount.unwrap(),
            }),
            "dispute" => Ok(Transaction::Dispute { tx_id: itx.tx_id }),
            "resolve" => Ok(Transaction::Resolve { tx_id: itx.tx_id }),
            "chargeback" => Ok(Transaction::Resolve { tx_id: itx.tx_id }),
            _ => Err(TransactionConversionError::UnknownTransactionType { tx_id: itx.tx_id }),
        }
    }
}

/// A Transaction from and input file. Not for use by clients.
#[derive(Deserialize, Debug)]
pub struct InputTransaction {
    #[serde(rename = "type")]
    tx_type: String,
    pub client: u16,
    #[serde(rename = "tx")]
    tx_id: u32,
    amount: Option<f32>,
}

#[derive(Error, Debug)]
pub enum TransactionConversionError {
    #[error("The type of transaction for tx {tx_id} could not be determined")]
    UnknownTransactionType { tx_id: u32 },
}
