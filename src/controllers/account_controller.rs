use crate::account::Account;
use crate::controllers::forms::Transaction;
use crate::controllers::presenters::AccountPresenter;
use actix_web::web;
use actix_web::{get, post};
use std::sync::RwLock;

#[get("/total")]
pub async fn get_total(state: web::Data<RwLock<Account>>) -> String {
    state.read().unwrap().total().to_string()
}

#[get("/account")]
pub async fn show(state: web::Data<RwLock<Account>>) -> web::Json<AccountPresenter> {
    let lock = state.read().unwrap();
    web::Json(lock.present())
}

#[post("/transfer")]
pub async fn transfer(
    state: web::Data<RwLock<Account>>,
    transaction: web::Json<Transaction>,
) -> web::Json<AccountPresenter> {
    let mut lock = state.write().unwrap();
    lock.add_transaction(transaction.into_inner());
    web::Json(lock.present())
}
