use rocket::{State, get};
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::sync::Arc;
use bollard::Docker;
use bollard::container::ListContainersOptions;
use bollard::container::LogsOptions;
use crate::rocket::futures::TryStreamExt;

#[get("/containers")]
pub async fn containers(docker: &State<Arc<Docker>>) -> Result<Template, String> {
    let options = Some(ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    });

    // Fetch list of containers
    let containers = docker.list_containers(options).await.map_err(|e| e.to_string())?;

    // Create a vector to hold container information
    let container_info: Vec<HashMap<String, String>> = containers.into_iter().map(|container| {
        let mut info = HashMap::new();
        // join on name due to multiple name potential
        info.insert("id".to_string(), container.id.unwrap_or_default());
        info.insert("names".to_string(), container.names.unwrap_or_default().join(","));
        info.insert("state".to_string(), container.state.unwrap_or_default());
        let image_name = container.image.unwrap_or_default();
        info.insert("image".to_string(), image_name.chars().take(50).collect());
        info

    }).collect();

    let mut context = HashMap::new();
    context.insert("containers", container_info);

    Ok(Template::render("containers", &context))
}

#[get("/containers/<id>")]
pub async fn container_details(docker: &State<Arc<Docker>>, id: &str) -> Result<Template, String> {
    let container_info = docker.inspect_container(&id, None).await.map_err(|e| e.to_string())?;
    println!("{:?}", container_info);
    let mut context = HashMap::<&str, String>::new();
    context.insert("id", container_info.id.unwrap_or_default());
    context.insert("name", container_info.name.unwrap_or_default());

    Ok(Template::render("container_details", &context))
}

#[get("/containers/<id>/logs")]
pub async fn container_logs(docker: &State<Arc<Docker>>, id: &str) -> Result<String, String> {
  let options = LogsOptions::<String> {
      follow: false,
      stdout: true,
      stderr: true,
      ..Default::default()
  };

  let logs = docker.logs(id, Some(options))
      .try_collect::<Vec<_>>() // Collect all log chunks into a Vec
      .await
      .map_err(|e| e.to_string())?;

  // Convert logs to a single String
  let logs_str = logs.into_iter()
      .map(|log| String::from_utf8_lossy(&log.into_bytes()).to_string())
      .collect::<Vec<String>>()
      .join("\n");

  Ok(logs_str)
}