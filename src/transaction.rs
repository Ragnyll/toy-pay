pub enum Transaction {
    Withdraw { tx_id: u32, amount: f32 },
    Deposit { tx_id: u32, amount: f32 },
    Dispute { tx_id: u32 },
    Resolve { tx_id: u32 },
    Chargeback { tx_id: u32 },
}
