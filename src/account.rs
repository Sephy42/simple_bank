use crate::forms::Transaction;
use crate::presenters::AccountPresenter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Account {
    pub transactions: Vec<Transaction>,
}

impl Account {
    pub fn total(&self) -> i32 {
        self.transactions
            .iter()
            .fold(0_i32, |acc, t| acc + t.signed_amount())
    }

    pub fn add_transaction(&mut self, t: Transaction) {
        self.transactions.push(t);
    }

    pub fn present(&self) -> AccountPresenter {
        AccountPresenter {
            transactions: self.transactions.clone(),
            total: self.total(),
        }
    }
}
