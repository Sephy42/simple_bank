use crate::controllers::forms::Transaction;
use crate::controllers::presenters::AccountPresenter;
use crate::presenters::StatePresenter;
use crate::{Account, State};
use actix_web::web;
use actix_web::{get, post};

#[get("/{id}")]
pub async fn show(
    id: web::Path<usize>,
    state: web::Data<State>,
) -> Option<web::Json<AccountPresenter>> {
    let lock = state.accounts.read().unwrap();
    let account = lock.get(*id)?;
    Some(web::Json(account.present(*id)))
}

#[get("")]
pub async fn show_all(state: web::Data<State>) -> web::Json<StatePresenter> {
    web::Json(state.present())
}

#[post("/{id}/transfer")]
pub async fn transfer(
    id: web::Path<usize>,
    state: web::Data<State>,
    transaction: web::Json<Transaction>,
) -> Option<web::Json<AccountPresenter>> {
    let mut lock = state.accounts.write().unwrap();
    let account = lock.get_mut(*id)?;
    account.add_transaction(transaction.into_inner());
    Some(web::Json(account.present(*id)))
}

#[post("")]
pub async fn create(state: web::Data<State>) -> web::Json<AccountPresenter> {
    let mut lock = state.accounts.write().unwrap();

    let new_account = Account::default();
    lock.push(new_account.clone());
    web::Json(new_account.present(lock.len() - 1))
}

// [x] POST /accounts -> Creer un account
// DELETE /accounts/{id} -> Supprimer un account
// [x] GET /accounts (INDEX) -> Recuperer tous les accounts
// [x] GET /accounts/{id} (SHOW) -> Recuperer un account
// [x] POST /accounts/{id}/transfer -> Effectuer une transaction
