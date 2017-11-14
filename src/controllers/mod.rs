use rocket::Rocket;

pub mod post;
pub mod category;
mod context;

/// Mounts all controller routes on the provided Rocket instance
pub fn mount(instance: Rocket) -> Rocket {
    instance
        .mount("/posts", post::all_routes())
        .mount("/categories",category::all_routes())
}