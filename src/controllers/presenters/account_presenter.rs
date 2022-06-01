use crate::forms::Transaction;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccountPresenter {
    pub id: usize,
    pub transactions: Vec<Transaction>,
    pub total: i32,
}
