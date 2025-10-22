use tauri_app_demo_lib::logging::{config, logger};
use tracing::{info, info_span, instrument, trace};

fn main(){
    // 初始化日志系统
    let config = config::LogConfig {
        level: "trace".to_string(),
        output: "both".to_string(),
        file_path: Some(".\\logs\\app.log".to_string()),
    };

    if let Err(e) = logger::init_logger(&config) {
        eprintln!("Failed to initialize logger: {}", e);
        return;
    };

    // 创建主 span
    let main_span = info_span!("main", version = "1.0.0", pid = std::process::id());
    let _guard = main_span.entered(); // 进入 span 作用域

    trace!("Application started");

    // 嵌套 span
    {
        let child_span = info_span!("initialization").entered();
        trace!("Initializing components");
        // 初始化代码...
    } // child_span 在这里自动退出

    process_data();

    trace!("Application finished");
}

#[instrument]
fn process_data() {
    trace!("Processing data");

    // 这个函数会自动创建 span，包含函数名和参数
    let result = calculate_something(42);
    trace!(result, "Calculation completed");
}

fn calculate_something(value: i32) -> i32 {
    info!("Calculating...");
    value * 2
}
