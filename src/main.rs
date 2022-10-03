use dropshot::{
    ApiDescription, ConfigDropshot, ConfigLogging, ConfigLoggingLevel, HttpServerStarter,
};

extern crate heartbeat_rs;
use heartbeat_rs::context::ServerContext;
use heartbeat_rs::endpoints;

#[tokio::main]
async fn main() -> Result<(), String> {
    let log = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    }
    .to_logger("heartbeat-api")
    .map_err(|e| e.to_string())?;

    let context = ServerContext {
        auth_token: String::from("SomeValue"),
    };

    // create new api description
    let mut api = ApiDescription::new();
    // register /api/beat
    api.register(endpoints::heartbeat_api_post_beat)?;
    // register /api/update/stats
    api.register(endpoints::heartbeat_api_update_post_stats)?;
    // register /api/update/devices
    api.register(endpoints::heartbeat_api_update_post_devices)?;
    // register /api/info
    api.register(endpoints::heartbeat_api_get_info)?;
    // register /api/stats
    api.register(endpoints::heartbeat_api_get_stats)?;
    // register /api/devices
    //api.register(endpoints::heartbeat_api_get_devices)?;

    let server = HttpServerStarter::new(
        &ConfigDropshot {
            bind_address: "0.0.0.0:3000".parse().unwrap(),
            request_body_max_bytes: 4096,
            tls: None,
        },
        api,
        context,
        &log,
    )
    .map_err(|error| format!("failed to start server: {}", error))?
    .start();

    server.await
}
