use rocket::Rocket;

pub mod post;

pub fn mount(instance: Rocket) -> Rocket {
    instance.mount("/posts", routes![post::new_post_get, post::new_post_post])
}