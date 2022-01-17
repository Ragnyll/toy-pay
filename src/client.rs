use thiserror::Error;
use crate::transaction::Transaction;
use std::collections::HashMap;

/// A light tracking of a transaction previously processed by the client
#[derive(Debug)]
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
#[derive(Debug)]
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
                Transaction::Deposit { tx_id, amount } => self.deposit(tx_id, amount)?,
                Transaction::Withdraw { tx_id, amount } => self.withdraw(tx_id, amount)?,
                Transaction::Dispute { tx_id } => self.dispute(tx_id)?,
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

    /// dispute a previous transaction
    fn dispute(&mut self, tx_id: u32) -> Result<(), TransactionError> {
        if let Some(tx) = self.transactions.get_mut(&tx_id) {
            if tx.is_disputed {
                return Err(TransactionError::PartnerDisputeError { tx_id });
            }
            tx.is_disputed = true;
            self.available += tx.amount;
            self.held += tx.amount;
            // Total funds remains constant

            return Ok(());
        }
        Err(TransactionError::PartnerDisputeError { tx_id })
    }

    /// resolves a disputed transaction
    fn resolve_dispute(&mut self, tx_id: u32) -> Result<(), TransactionError> {
        if let Some(tx) = self.transactions.get_mut(&tx_id) {
            if !tx.is_disputed {
                return Err(TransactionError::PartnerResolveError { tx_id });
            }
            tx.is_disputed = false;
            self.available += tx.amount;
            self.held -= tx.amount;
            // Total funds remains constant

            return Ok(());
        }
        Err(TransactionError::PartnerResolveError { tx_id })
    }

    fn chargeback(&mut self, tx_id: u32) -> Result<(), TransactionError> {
        if let Some(tx) = self.transactions.get_mut(&tx_id) {
            if !tx.is_disputed {
                return Err(TransactionError::PartnerChargebackError { tx_id });
            }
            self.is_locked = true;
            self.held -= tx.amount;
            self.total_funds -= tx.amount;
            // Total funds remains constant

            return Ok(());
        }
        Err(TransactionError::PartnerChargebackError { tx_id })
    }
}

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("The amount specified by the transaction is invalid: {amount:?}")]
    InvalidAmountError { amount: f32 },
    #[error("The dispute raised against transaction {tx_id:?} is invalid")]
    PartnerDisputeError { tx_id: u32 },
    #[error("The Resolve raised against transaction {tx_id:?} is invalid")]
    PartnerResolveError { tx_id: u32 },
    #[error("The Chargeback raised against transaction {tx_id:?} is invalid")]
    PartnerChargebackError { tx_id: u32 },
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

    #[test]
    // TODO COME BACK TO THINK ABOUT REVERSING! disputing a deposit seems like it should decrease
    // funds
    fn succesful_dispute() {
        let mut client = Client::new(1u16);
        client.deposit(1, 32.0).unwrap();
        client.dispute(1).unwrap();
        assert_eq!(client.available, 64.0);
        assert_eq!(client.held, 32.0);
        assert_eq!(client.transactions.get(&1).unwrap().is_disputed, true);
    }

    #[test]
    fn disputed_transaction_dne() {
        let mut client = Client::new(1u16);
        assert_eq!(client.dispute(1).is_err(), true);
    }

    #[test]
    fn disputed_transaction_already_disputed() {
        let mut client = Client::new(1u16);
        client.deposit(1, 32.0).unwrap();
        assert_eq!(client.dispute(1).is_ok(), true);
        assert_eq!(client.dispute(1).is_err(), true);
    }

    #[test]
    fn successful_resolve_dispute() {
        let mut client = Client::new(1u16);
        client.deposit(1, 32.0).unwrap();
        client.dispute(1).unwrap();
        assert_eq!(client.resolve_dispute(1).is_ok(), true);
        assert_eq!(client.held, 0.0);
        assert_eq!(client.available, 96.0);
    }

    #[test]
    fn transaction_to_resolve_dne() {
        let mut client = Client::new(1u16);
        assert_eq!(client.resolve_dispute(1).is_err(), true);
    }

    #[test]
    fn transaction_to_resolve_not_under_dispute() {
        let mut client = Client::new(1u16);
        client.deposit(1, 32.0).unwrap();
        assert_eq!(client.resolve_dispute(1).is_err(), true);
    }

    #[test]
    fn succesful_chargeback() {
        let mut client = Client::new(1u16);
        client.deposit(1, 32.0).unwrap();
        client.dispute(1).unwrap();
        client.chargeback(1).unwrap();
        assert_eq!(client.is_locked, true);
        assert_eq!(client.held, 0.0);
        assert_eq!(client.total_funds, 0.0);
    }

    #[test]
    fn transaction_to_chargeback_dne() {
        let mut client = Client::new(1u16);
        assert_eq!(client.chargeback(1).is_err(), true);
    }

    #[test]
    fn transaction_to_chargeback_not_under_dispute() {
        let mut client = Client::new(1u16);
        client.deposit(1, 32.0).unwrap();
        assert_eq!(client.chargeback(1).is_err(), true);
    }
}
