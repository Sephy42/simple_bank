use crate::Account;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug, Default)]
pub struct State {
    pub accounts: RwLock<HashMap<usize, Account>>,
}
