#[macro_use] extern crate rocket;
use bollard::Docker;
use bollard::container::ListContainersOptions;

use rocket_dyn_templates::Template;
use std::collections::HashMap;

use std::sync::Arc;
use rocket::State;

#[get("/")]
async fn index(docker: &State<Arc<Docker>>) -> Result<Template, String> {
    let info = docker.info().await.map_err(|e| e.to_string())?;

    let mut context = HashMap::<&str, String>::new();

    context.insert("containers", info.containers.unwrap_or(0).to_string());
    context.insert("containers_running", info.containers_running.unwrap_or(0).to_string());
    context.insert("images", info.images.unwrap_or(0).to_string());
    context.insert("operating_system", info.operating_system.unwrap_or_default().to_string());
    context.insert("ncpu", info.ncpu.unwrap_or_default().to_string());
    let memory_gb = info.mem_total.unwrap_or(0) as f64 / 1_073_741_824_f64;
    context.insert("memory", format!("{:.2} GB", memory_gb));
    println!("{:?}", context);

    Ok(Template::render("index", &context))
}

#[get("/containers")]
async fn containers(docker: &State<Arc<Docker>>) -> Result<Template, String> {
    let options = Some(ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    });

    // Fetch list of containers
    let containers = docker.list_containers(options).await.map_err(|e| e.to_string())?;

    // Create a vector to hold container information
    let container_info: Vec<HashMap<String, String>> = containers.into_iter().map(|container| {
        let mut info = HashMap::new();
        info.insert("id".to_string(), container.id.unwrap_or_default());
        // join on name due to multiple name potential
        info.insert("names".to_string(), container.names.unwrap_or_default().join(","));
        info.insert("state".to_string(), container.state.unwrap_or_default());
        info

    }).collect();

    let mut context = HashMap::new();
    context.insert("containers", container_info);

    Ok(Template::render("containers", &context))
}

#[launch]
fn rocket() -> _ {
    let docker = Docker::connect_with_unix_defaults().expect("Failed to connec to Docker");
    let docker = Arc::new(docker);
    
    println!("Starting Rocket application...");
    rocket::build()
    // .mount("/", routes![index])
    .mount("/", routes![index, containers])
    .mount("/static", rocket::fs::FileServer::from("static"))
    .attach(Template::fairing())
    .manage(docker)
}