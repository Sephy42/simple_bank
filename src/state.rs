use crate::Account;
use std::sync::RwLock;

#[derive(Debug, Default)]
pub struct State {
    pub accounts: RwLock<Vec<Account>>,
}
