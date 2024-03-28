use rocket::{State, get};
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::sync::Arc;
use bollard::Docker;

#[get("/")]
pub async fn index(docker: &State<Arc<Docker>>) -> Result<Template, String> {
    let info = docker.info().await.map_err(|e| e.to_string())?;

    let mut context = HashMap::<&str, String>::new();

    context.insert("containers", info.containers.unwrap_or(0).to_string());
    context.insert("containers_running", info.containers_running.unwrap_or(0).to_string());
    context.insert("images", info.images.unwrap_or(0).to_string());
    context.insert("operating_system", info.operating_system.unwrap_or_default().to_string());
    context.insert("ncpu", info.ncpu.unwrap_or_default().to_string());
    let memory_gb = info.mem_total.unwrap_or(0) as f64 / 1_073_741_824_f64;
    context.insert("memory", format!("{:.2} GB", memory_gb));

    Ok(Template::render("index", &context))
}