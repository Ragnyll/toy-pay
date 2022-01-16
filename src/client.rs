use crate::transaction::Transaction;
use std::collections::HashMap;

/// A client who can access and manipulate their funds
pub struct Client {
    id: u16,
    transactions: HashMap<u32, Transaction>,
    is_locked: bool,
    available: f32,
    held: f32,
    total_funds: f32,
}

impl Client {
    /// Creates a new client. Primarily for use when there is no
    pub fn new(id: u16) -> Self {
        Self {
            id: id,
            transactions: HashMap::new(),
            is_locked: false,
            available: 0.0,
            held: 0.0,
            total_funds: 0.0,
        }
    }

    /// Updates the client based on the tx if it is not locked
    pub fn process_transaction(&mut self, tx: Transaction) {
        if !self.is_locked {
            match tx.tx_type {
                // TODO: Handle all tx types
                _ => (),
            }
            self.transactions.insert(tx.tx_id, tx);
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::transaction::{Transaction, TransactionType};
    use super::Client;

    #[test]
    fn locked_client_does_not_update() {
        let mut client = Client::new(1u16);
        client.is_locked = true;
        let tx = Transaction::new(350, TransactionType::DEPOSIT, Some(123.45));
        client.process_transaction(tx);
        assert_eq!(client.total_funds, 0.0)
    }

    #[test]
    fn valid_transaction_processed() {
        // a processed transaction should be added to the clients transactions
        let mut client = Client::new(1u16);
        let tx = Transaction::new(350, TransactionType::DEPOSIT, Some(123.45));
        client.process_transaction(tx);
        assert_eq!(client.transactions.len(), 1)
    }
}
