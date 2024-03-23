#[macro_use] extern crate rocket;
use bollard::Docker;
use bollard::container::ListContainersOptions;

use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    let context = HashMap::<&str, &str>::new();
    Template::render("index", &context)
}

#[get("/containers")]
async fn containers() -> Result<Template, String> {
    // Initialize Bollard Docker connection
    let docker = Docker::connect_with_unix_defaults().map_err(|e| e.to_string())?;

    // Options to list all containers, not just running ones
    let options = Some(ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    });

    // Fetch list of containers
    let containers = docker.list_containers(options).await.map_err(|e| e.to_string())?;

    // Create a string with all container IDs
    let ids: Vec<String> = containers.into_iter()
        .map(|container| container.id.unwrap_or_default())
        .collect();

    let mut context = HashMap::new();
    context.insert("containers", ids);

    Ok(Template::render("containers", &context))
}

#[launch]
fn rocket() -> _ {
    println!("Starting Rocket application...");
    rocket::build()
    // .mount("/", routes![index])
    .mount("/", routes![index, containers])
    .attach(Template::fairing())
}