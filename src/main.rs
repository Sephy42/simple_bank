#![warn(clippy::all, clippy::nursery)]
mod account;
mod controllers;
mod state;

use crate::account::Account;
use crate::account_controller;
use crate::state::State;
use actix_web::dev::Service;
use actix_web::{web, App, HttpServer};
use controllers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let state = web::Data::new(State::default());
    HttpServer::new(move || {
        App::new().app_data(state.clone()).service(
            web::scope("/accounts")
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
                .service(account_controller::transfer)
                .service(account_controller::show)
                .service(account_controller::create),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
