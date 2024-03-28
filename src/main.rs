#[macro_use] extern crate rocket;
use bollard::Docker;
use rocket_dyn_templates::Template;
use std::sync::Arc;

mod routes;

#[launch]
fn rocket() -> _ {
    let docker = Docker::connect_with_unix_defaults().expect("Failed to connec to Docker");
    let docker = Arc::new(docker);
    
    println!("Starting Rocket application...");
    rocket::build()
    // .mount("/", routes![index])
    .mount("/", routes![
        routes::index::index, 
        routes::containers::containers, 
        routes::containers::container_details,
        routes::containers::container_logs
    ])
    .mount("/static", rocket::fs::FileServer::from("static"))
    .attach(Template::fairing())
    .manage(docker)
}