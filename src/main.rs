use axum::{response::Html, response::Json, routing::get, Router};
use humantime::format_duration;
use serde_json::json;
use std::sync::Arc;
use sysinfo::System;
use tokio::fs;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tracing::{debug, error, info};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let shared_system = Arc::new(Mutex::new(System::new_all()));
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/cpu", get(cpu_handler))
        .route("/mem", get(memory_handler))
        .route("/system", get(system_handler))
        .route("/networks", get(network_handler))
        .route("/proc", get(proc_handler))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(shared_system.clone());

    info!("Server starting on 0.0.0.0:3000");

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(l) => {
            info!("Successfully bound to port 3000");
            l
        }
        Err(e) => {
            error!("Failed to bind to port 3000: {e}");
            panic!("Error {e}");
        }
    };

    match axum::serve(listener, app).await {
        Ok(_) => info!("Server exited cleanly"),
        Err(e) => {
            error!("Server encountered an error: {e}");
        }
    };
}

async fn root_handler() -> Result<Html<String>, axum::http::StatusCode> {
    debug!("Handling root request");
    (fs::read_to_string("templates/index.html").await).map_or(
        Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        |content| {
            info!("Successfully served index.html");
            Ok(Html(content))
        },
    )
}

async fn cpu_handler(state: axum::extract::State<Arc<Mutex<System>>>) -> Json<serde_json::Value> {
    debug!("Handling CPU stats request");
    let mut sys = state.lock().await;
    sys.refresh_cpu_all();
    let cpu_usage = sys
        .cpus()
        .iter()
        .map(sysinfo::Cpu::cpu_usage)
        .collect::<Vec<_>>();
    let load_average = System::load_average().one;

    info!(
        "CPU stats retrieved: usage={:?}, load_average={}",
        cpu_usage, load_average
    );

    Json(json!({
        "cpu_usage": cpu_usage,
        "load_average": load_average,
    }))
}

async fn memory_handler(
    state: axum::extract::State<Arc<Mutex<System>>>,
) -> Json<serde_json::Value> {
    debug!("Handling memory stats request");
    let mut sys = state.lock().await;
    sys.refresh_memory();
    info!(
        "Memory stats retrieved: total={}, used={}, free={}",
        sys.total_memory(),
        sys.used_memory(),
        sys.free_memory()
    );

    Json(json!({
        "total_memory": sys.total_memory(),
        "used_memory": sys.used_memory(),
        "free_memory": sys.free_memory(),
        "available_memory": sys.available_memory(),
        "total_swap": sys.total_swap(),
        "used_swap": sys.used_swap(),
        "free_swap": sys.free_swap(),
    }))
}

async fn system_handler() -> Json<serde_json::Value> {
    debug!("Handling system stats request");
    let system_info = json!({
        "name": System::name().unwrap_or_else(|| "<unknown>".to_owned()),
        "kernel_version": System::kernel_version().unwrap_or_else(|| "<unknown>".to_owned()),
        "os_version": System::os_version().unwrap_or_else(|| "<unknown>".to_owned()),
        "long_os_version": System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned()),
        "distribution_id": System::distribution_id(),
        "host_name": System::host_name().unwrap_or_else(|| "<unknown>".to_owned()),
        "uptime": format_duration(std::time::Duration::new((System::uptime() as i64).try_into().unwrap_or_default(), 0)).to_string(),
    });

    info!("System stats retrieved: {:?}", system_info);
    Json(system_info)
}

async fn network_handler() -> Json<serde_json::Value> {
    debug!("Handling network stats request");
    let networks = sysinfo::Networks::new_with_refreshed_list()
        .iter()
        .map(|(name, data)| {
            json!({
                "interface_name": name,
                "received": data.total_received(),
                "transmitted": data.total_transmitted(),
            })
        })
        .collect::<Vec<_>>();

    info!("Network stats retrieved: {:?}", networks);
    Json(json!({ "networks": networks }))
}

async fn proc_handler(state: axum::extract::State<Arc<Mutex<System>>>) -> Json<serde_json::Value> {
    debug!("Handling processes stats request");
    let mut sys = state.lock().await;
    sys.refresh_all(); // Ensure the processes are updated

    let processes_info = sys
        .processes()
        .iter()
        .map(|(pid, process)| {
            json!({
                "pid": *pid.to_string(),
                "name": process.name().to_string_lossy(),
                "memory": process.memory(),
                "virtual_memory": process.virtual_memory(),
                "cpu_usage": process.cpu_usage(),
                "run_time": process.run_time(),
                "status": format!("{:?}", process.status())
            })
        })
        .collect::<Vec<_>>();

    info!(
        "Processes stats retrieved: {} processes",
        processes_info.len()
    );
    Json(json!({
        "processes": processes_info,
    }))
}
