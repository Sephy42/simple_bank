use crate::presenters::AccountPresenter;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StatePresenter {
    pub accounts: Vec<AccountPresenter>,
}
