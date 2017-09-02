use rocket::Rocket;

pub mod post;

/// Mounts all controller routes on the provided Rocket instance
pub fn mount(instance: Rocket) -> Rocket {
    instance.mount("/posts", post::all_routes())
}