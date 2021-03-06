// Copyright (C) 2020 Peter Mezei
//
// This file is part of Gardenzilla.
//
// Gardenzilla is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Gardenzilla is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Gardenzilla.  If not, see <http://www.gnu.org/licenses/>.

#![feature(proc_macro_hygiene, decl_macro, plugin)]

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate core_lib;
extern crate crypto;
extern crate ifeq;
extern crate jwt;
extern crate num_format;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_derive;
extern crate storaget;

pub mod controller;
pub mod cors;
pub mod guard;
pub mod login;
pub mod model;
pub mod prelude;

use crate::prelude::*;
use core_lib::model::*;
use guard::*;
use rocket::Request;
use rocket_cors::AllowedHeaders;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Mutex;
use storaget::*;

#[get("/")]
fn index() -> String {
    "Gardenzilla Hello".to_owned()
}

#[derive(Debug, Serialize)]
struct ApiWelcomeSchema {
    message: &'static str,
}

#[get("/")]
fn api_welcome(_user: Login) -> StatusOk<ApiWelcomeSchema> {
    StatusOk(ApiWelcomeSchema {
        message: "Welcome to Gardenzilla API",
    })
}

#[catch(404)]
fn not_found(_: &Request<'_>) -> ApiError {
    ApiError::NotFound
}

#[catch(401)]
fn unauthorized(_: &Request<'_>) -> ApiError {
    ApiError::Unauthorized
}

#[catch(422)]
fn form_error(_: &Request<'_>) -> ApiError {
    ApiError::InternalError("Minden mező kitöltése kötelező!".to_owned())
}

fn rocket(data: DataLoad) -> rocket::Rocket {
    let mut methods = std::collections::HashSet::new();
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Post));
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Get));
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Put));
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Delete));
    methods.insert(rocket_cors::Method::from(rocket::http::Method::Options));

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins: rocket_cors::AllOrSome::All,
        allowed_methods: methods,
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::ignite()
        .attach(cors)
        // .attach(CORS())
        .manage(data)
        .mount("/", routes![index])
        .mount(
            "/api",
            routes![
                controller::login::post,
                controller::login::reset_password,
                controller::profile::profile_get,
                controller::profile::profile_post,
                controller::profile::password_change,
                controller::user::user_all_get,
                controller::user::user_id_get,
                controller::user::user_new_post,
                controller::cash::cash_register_all_get,
                controller::cash::cash_register_last_n_get,
                controller::cash::cash_register_new_put,
                controller::cash::cash_register_daterange_get,
                controller::cash::cash_register_till_datetime_get
            ],
        )
        .register(catchers![not_found, unauthorized, form_error])
}

pub struct DataLoad {
    users: Mutex<VecPack<User>>,
    cash_register: Mutex<Pack<CashRegister>>,
}

fn main() -> PackResult<()> {
    let data = DataLoad {
        users: Mutex::new(VecPack::try_load_or_init(PathBuf::from("data/users"))?),
        cash_register: Mutex::new(Pack::try_load_or_init(
            PathBuf::from("data/cash_register"),
            "default",
        )?),
    };
    rocket(data).launch();
    Ok(())
}
