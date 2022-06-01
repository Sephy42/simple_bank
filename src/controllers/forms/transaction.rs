use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TransactionDirection {
    Credit,
    Withdrawal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub amount: u32,
    pub direction: TransactionDirection,
    pub description: String,
}

impl Transaction {
    pub const fn signed_amount(&self) -> i32 {
        match self.direction {
            TransactionDirection::Credit => self.amount as i32,
            TransactionDirection::Withdrawal => -(self.amount as i32),
        }
    }
}
