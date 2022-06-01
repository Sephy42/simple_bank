use crate::presenters::{AccountPresenter, StatePresenter};
use crate::Account;
use std::sync::RwLock;

#[derive(Debug, Default)]
pub struct State {
    pub accounts: RwLock<Vec<Account>>,
}

impl State {
    pub fn present(&self) -> StatePresenter {
        let lock = self.accounts.read().unwrap();
        let mut vec_accounts: Vec<AccountPresenter> = Vec::new();
        for (i, account) in lock.iter().enumerate() {
            vec_accounts.push(account.present(i));
        }
        StatePresenter {
            accounts: vec_accounts,
        }
    }
}
