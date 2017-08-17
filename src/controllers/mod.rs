use rocket::Rocket;

pub mod post;

pub fn mount(instance: Rocket) -> Rocket {
    instance.mount("/posts", post::all_routes())
}