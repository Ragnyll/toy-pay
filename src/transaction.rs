use std::convert::From;

pub enum TransactionType {
    WITHDRAW,
    DEPOSIT,
    DISPUTE,
    RESOLVE,
    CHARGEBACK,
    /// Transactions with an unknown TransactionType should not be created
    UNKNOWN,
}

impl From<&str> for TransactionType {
    fn from(item: &str) -> Self {
        match item {
            "withdraw" => Self::WITHDRAW,
            "deposit" => Self::DEPOSIT,
            "dispute" => Self::DISPUTE,
            "resolve" => Self::RESOLVE,
            "chargeback" => Self::CHARGEBACK,
            _ => Self::UNKNOWN,
        }
    }
}

#[readonly::make]
pub struct Transaction {
    #[readonly]
    pub tx_id: u32,
    #[readonly]
    pub tx_type: TransactionType,
    #[readonly]
    pub amount: Option<f32>,
    pub is_disputed: bool,
}

impl Transaction {
    pub fn new(tx_id: u32, tx_type: TransactionType, amount: Option<f32>) -> Self {
        Self {
            tx_id: tx_id,
            tx_type: tx_type,
            amount: amount,
            is_disputed: false,
        }
    }
}
