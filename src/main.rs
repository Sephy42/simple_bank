#![warn(clippy::all, clippy::nursery)]
mod account;
mod controllers;

use crate::account::Account;
use crate::account_controller;
use actix_web::dev::Service;
use actix_web::{web, App, HttpServer};
use controllers::*;
use std::sync::RwLock;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let account = web::Data::new(RwLock::new(Account::default()));
    HttpServer::new(move || {
        App::new().app_data(account.clone()).service(
            web::scope("/api")
                .wrap_fn(|req, srv| {
                    log::info!("{} {}", req.method(), req.path());
                    let resp = srv.call(req);
                    async {
                        match resp.await {
                            Err(error) => {
                                log::error!("{}", error);
                                log::debug!("{:?}", error);
                                Err(error)
                            }
                            Ok(response) => {
                                log::debug!("Rendering {}", response.status());
                                Ok(response)
                            }
                        }
                    }
                })
                .service(account_controller::get_total)
                .service(account_controller::transfer)
                .service(account_controller::show),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
