//! The controllers providing the endpoints for the application

use rocket::Rocket;

pub mod category;
pub mod period;
pub mod transaction;
mod context;

/// Mounts all controller routes on the provided Rocket instance
pub fn mount(instance: Rocket) -> Rocket {
    instance
        .mount("/categories", category::all_routes())
        .mount("/periods", period::all_routes())
        .mount("/transactions", transaction::all_routes())
}
