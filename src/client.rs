use thiserror::Error;
use crate::transaction::Transaction;
use std::collections::HashMap;

/// A light tracking of a transaction previously processed by the client
struct PreviousTransaction {
    amount: f32,
    is_disputed: bool,
}

impl PreviousTransaction {
    fn new(amount: f32) -> Self {
        Self {
            amount,
            // A newly processed cannot automatically be in dispute but can be disputed later
            is_disputed: false,
        }
    }
}

/// A client who can access and manipulate their funds
pub struct Client {
    id: u16,
    transactions: HashMap<u32, PreviousTransaction>,
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
    pub fn process_transaction(&mut self, tx: Transaction) -> Result<(), TransactionError> {
        if !self.is_locked {
            match tx {
                // TODO: Handle all tx types
                Transaction::Deposit { tx_id, amount } => self.deposit(tx_id, amount)?,
                Transaction::Withdraw { tx_id, amount } => self.withdraw(tx_id, amount)?,

                _ => (),
            };
        }

        Ok(())
    }

    /// handle a deposit transaction
    fn deposit(&mut self, tx_id: u32, amount: f32) -> Result<(), TransactionError> {
        if amount < 0.0 {
            return Err(TransactionError::InvalidAmountError { amount });
        }
        self.available += amount;
        self.total_funds += amount;
        self.transactions
            .insert(tx_id, PreviousTransaction::new(amount));

        Ok(())
    }

    /// handle a withdrawl transaction
    fn withdraw(&mut self, tx_id: u32, amount: f32) -> Result<(), TransactionError> {
        if amount < 0.0 {
            return Err(TransactionError::InvalidAmountError { amount });
        }
        self.available -= amount;
        self.total_funds -= amount;
        self.transactions
            .insert(tx_id, PreviousTransaction::new(amount));

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("The amount specified by the transaction is invalid: {amount:?}")]
    InvalidAmountError { amount: f32 },
}

#[cfg(test)]
pub mod test {
    use crate::transaction::Transaction;
    use super::Client;

    #[test]
    fn locked_client_does_not_update() {
        let mut client = Client::new(1u16);
        client.is_locked = true;
        let tx = Transaction::Deposit {
            tx_id: 1,
            amount: 32.0,
        };
        client.process_transaction(tx).unwrap();
        assert_eq!(client.total_funds, 0.0)
    }

    #[test]
    fn valid_transaction_processed() {
        // a processed transaction should be added to the clients transactions
        let mut client = Client::new(1u16);
        let tx = Transaction::Deposit {
            tx_id: 1,
            amount: 32.0,
        };
        client.process_transaction(tx).unwrap();
        assert_eq!(client.transactions.len(), 1)
    }

    #[test]
    fn valid_deposit() {
        let mut client = Client::new(1u16);
        client.deposit(1, 32.0).unwrap();
        assert_eq!(client.available, 32.0);
        assert_eq!(client.total_funds, 32.0);
    }

    #[test]
    fn negative_deposit() {
        let mut client = Client::new(1u16);
        assert_eq!(client.deposit(1, -32.0).is_err(), true);
        assert_eq!(client.available, 0.0);
    }

    #[test]
    fn valid_withdrawl() {
        let mut client = Client::new(1u16);
        client.withdraw(1, 32.0).unwrap();
        assert_eq!(client.available, -32.0);
        assert_eq!(client.total_funds, -32.0);
    }

    #[test]
    fn negative_withdrawl() {
        let mut client = Client::new(1u16);
        assert_eq!(client.withdraw(1, -32.0).is_err(), true);
        assert_eq!(client.available, 0.0);
    }
}
