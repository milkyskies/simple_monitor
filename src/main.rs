use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};
use dotenv::dotenv;
use nvml_wrapper::{Device, Nvml};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sysinfo::System;
use tokio::time::{sleep, Duration};

#[derive(Serialize, Deserialize, Clone)]
struct SystemStats {
    cpu_usage: CpuUsage,
    memory_usage: MemoryUsage,
    gpu_usage: Option<GpuUsage>,
    timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct CpuUsage {
    cores_total: usize,
    cores_usage: Vec<f32>,
    average_usage_percentage: f32,
    brand: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct MemoryUsage {
    used_bytes: u64,
    total_bytes: u64,
    used_percentage: f32,
}

#[derive(Serialize, Deserialize, Clone)]
struct GpuUsage {
    name: String,
    memory_used_bytes: u64,
    memory_total_bytes: u64,
    memory_used_percentage: f32,
    utilization_percentage: u32,
    temperature_celsius: u32,
}

struct AppState {
    system: System,
    nvml: Option<Nvml>,
}

async fn get_system_stats(
    State(state): State<Arc<tokio::sync::Mutex<AppState>>>,
) -> Result<Json<SystemStats>, StatusCode> {
    let mut app_state = state.lock().await;

    // Update system information
    app_state.system.refresh_all();

    // Wait a bit for CPU usage calculation
    sleep(Duration::from_millis(100)).await;
    app_state.system.refresh_cpu_all();

    // Get detailed CPU usage information
    let cpus = app_state.system.cpus();
    let cores_usage: Vec<f32> = cpus.iter().map(|cpu| cpu.cpu_usage()).collect();
    let average_usage = cores_usage.iter().sum::<f32>() / cores_usage.len() as f32;

    let cpu_usage = CpuUsage {
        cores_total: cores_usage.len(),
        cores_usage,
        average_usage_percentage: average_usage,
        brand: cpus
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown".to_string()),
    };

    // Get memory usage
    let memory_usage = MemoryUsage {
        used_bytes: app_state.system.used_memory(),
        total_bytes: app_state.system.total_memory(),
        used_percentage: (app_state.system.used_memory() as f32
            / app_state.system.total_memory() as f32)
            * 100.0,
    };

    // Get GPU usage if available
    let gpu_usage = if let Some(ref nvml) = app_state.nvml {
        match get_gpu_stats(nvml) {
            Ok(gpu) => Some(gpu),
            Err(e) => {
                eprintln!("Failed to get GPU stats: {}", e);
                None
            }
        }
    } else {
        None
    };

    let stats = SystemStats {
        cpu_usage,
        memory_usage,
        gpu_usage,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    Ok(Json(stats))
}

fn get_gpu_stats(nvml: &Nvml) -> Result<GpuUsage, Box<dyn std::error::Error>> {
    let device_count = nvml.device_count()?;

    if device_count == 0 {
        return Err("No NVIDIA devices found".into());
    }

    // Get the first (and only) GPU device
    let device: Device = nvml.device_by_index(0)?;

    let name = device.name()?;
    let memory_info = device.memory_info()?;
    let utilization = device.utilization_rates()?;
    let temperature =
        device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)?;

    Ok(GpuUsage {
        name,
        memory_used_bytes: memory_info.used,
        memory_total_bytes: memory_info.total,
        memory_used_percentage: (memory_info.used as f32 / memory_info.total as f32) * 100.0,
        utilization_percentage: utilization.gpu,
        temperature_celsius: temperature,
    })
}

async fn health_check() -> &'static str {
    "System Monitor is running!"
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file if it exists
    dotenv().ok();

    println!("Starting Simple System Monitor...");

    // Initialize system information
    let mut system = System::new_all();
    system.refresh_all();

    // Try to initialize NVML for GPU monitoring
    let nvml = match Nvml::init() {
        Ok(nvml) => {
            println!("NVIDIA GPU monitoring initialized successfully");
            Some(nvml)
        }
        Err(e) => {
            println!(
                "Failed to initialize NVIDIA GPU monitoring: {}. GPU stats will not be available.",
                e
            );
            None
        }
    };

    let app_state = Arc::new(tokio::sync::Mutex::new(AppState { system, nvml }));

    // Build our application with routes
    let app = Router::new()
        .route("/", get(health_check))
        .route("/stats", get(get_system_stats))
        .with_state(app_state);

    // Get host and port from environment variables or use defaults
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let bind_address = format!("{}:{}", host, port);

    // Run the server
    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .expect("Failed to bind to address");

    println!("System Monitor server running on http://{}", bind_address);
    println!("Available endpoints:");
    println!("  GET /       - Health check");
    println!("  GET /stats  - System statistics (CPU, Memory, GPU)");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
