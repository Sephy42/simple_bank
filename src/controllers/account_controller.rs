use crate::controllers::forms::Transaction;
use crate::controllers::presenters::AccountPresenter;
use crate::{Account, State};
use actix_web::web;
use actix_web::{delete, get, post};

#[get("/{id}")]
pub async fn show(
    id: web::Path<usize>,
    state: web::Data<State>,
) -> Option<web::Json<AccountPresenter>> {
    let lock = state.accounts.read().unwrap();
    let account = lock.get(&*id)?;
    Some(web::Json(account.present(*id)))
}

#[get("")]
pub async fn index(state: web::Data<State>) -> web::Json<Vec<AccountPresenter>> {
    let lock = state.accounts.read().unwrap();
    let res = lock
        .iter()
        .map(|(i, account)| account.present(*i))
        .collect();
    web::Json(res)
}

#[post("/{id}/transfer")]
pub async fn transfer(
    id: web::Path<usize>,
    state: web::Data<State>,
    transaction: web::Json<Transaction>,
) -> Option<web::Json<AccountPresenter>> {
    let mut lock = state.accounts.write().unwrap();
    let account = lock.get_mut(&*id)?;
    account.add_transaction(transaction.into_inner());
    Some(web::Json(account.present(*id)))
}

#[post("")]
pub async fn create(state: web::Data<State>) -> web::Json<AccountPresenter> {
    let mut lock = state.accounts.write().unwrap();

    let new_account = Account::default();
    let new_id = lock.keys().max().copied().unwrap_or(0) + 1;
    lock.insert(new_id, new_account.clone());
    web::Json(new_account.present(new_id))
}

#[delete("/{id}")]
pub async fn delete(
    id: web::Path<usize>,
    state: web::Data<State>,
) -> Option<web::Json<AccountPresenter>> {
    let mut lock = state.accounts.write().unwrap();
    let account = lock.remove(&*id)?;
    Some(web::Json(account.present(*id)))
}
