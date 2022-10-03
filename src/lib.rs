pub mod context {
    use http::header::HeaderValue;

    #[derive(Debug)]
    pub struct ServerContext {
        // any common configuration needs to go here
        pub auth_token: String,
    }
    impl ServerContext {
        pub fn new(auth: String) -> ServerContext {
            ServerContext {
                auth_token: String::from("CHANGEME"),
            }
        }
        pub fn is_authed(&self, auth: &HeaderValue) -> Result<(), String> {
            if self.auth_token.as_bytes() == auth.as_bytes() {
                Ok(())
            } else {
                Err(String::from("Token Invalid"))
            }
        }
    }
}

mod model {
    use schemars::JsonSchema;
    use serde::Serialize;

    #[derive(Serialize, JsonSchema)]
    pub struct Info {
        pub last_seen: String,
        pub time_difference: String,
        pub missing_beat: String,
        pub total_beats: String,
    }
    impl Info {
        pub fn new() -> Info {
            Info {
                last_seen: String::from(""),
                time_difference: String::from(""),
                missing_beat: String::from(""),
                total_beats: String::from(""),
            }
        }
    }

    #[derive(Serialize, JsonSchema)]
    pub struct Beat {
        pub device_name: String,
        pub timestamp: i64,
    }

    #[derive(Serialize, JsonSchema)]
    pub struct Device {
        pub device_name: String,
        pub last_beat: Beat,
        pub total_beats: i64,
        pub longest_missing_beat: i64,
    }

    #[derive(Serialize, JsonSchema)]
    pub struct Stats {
        #[serde(default)]
        pub last_beat_formatted: Option<String>,
        #[serde(default)]
        pub total_devices_formatted: Option<String>,
        #[serde(default)]
        pub total_visits_formatted: Option<String>,
        #[serde(default)]
        pub total_uptime_formatted: Option<String>,
        #[serde(default)]
        pub total_beats_formatted: Option<String>,
        pub total_visits: i64,
        pub total_uptime_milli: i64,
        pub total_beats: i64,
        pub longest_missing_beat: i64,
    }
}

pub mod endpoints {
    use super::context::ServerContext;
    use dropshot::{endpoint, HttpError, HttpResponseOk, RequestContext};
    use std::sync::Arc;

    #[endpoint {
        method = POST,
        path = "/api/beat",
    }]
    pub async fn heartbeat_api_post_beat(
        rqctx: Arc<RequestContext<ServerContext>>,
    ) -> Result<HttpResponseOk<i64>, HttpError> {
        let request = rqctx.request.lock().await;
        let context = rqctx.context();
        let auth = request.headers().get("Auth").unwrap();
        match context.is_authed(auth) {
            Ok(_) => match request.headers().get("Device") {
                Some(dev) => {
                    Ok(HttpResponseOk(0))
                }
                None => Err(HttpError::for_bad_request(
                    Some(String::from("Bad Request")),
                    String::from("Device header missing"),
                )),
            },
            Err(e) => Err(HttpError::for_bad_request(
                Some(String::from("Unauthenticated")),
                e,
            )),
        }
    }

    #[endpoint {
        method = POST,
        path = "/api/update/stats",
    }]
    pub async fn heartbeat_api_update_post_stats(
        rqctx: Arc<RequestContext<ServerContext>>,
    ) -> Result<HttpResponseOk<super::model::Stats>, HttpError> {
        let request = rqctx.request.lock().await;
        let context = rqctx.context();
        let auth = request.headers().get("Auth").unwrap();
        match context.is_authed(&auth) {
            Ok(_) => Ok(HttpResponseOk(super::model::Stats {
                last_beat_formatted: None,
                total_devices_formatted: None,
                total_visits_formatted: None,
                total_uptime_formatted: None,
                total_beats_formatted: None,
                total_visits: 0,
                total_uptime_milli: 0,
                total_beats: 0,
                longest_missing_beat: 0,
            })),
            Err(e) => Err(HttpError::for_bad_request(
                Some(String::from("Unauthenticated")),
                e,
            )),
        }
    }

    #[endpoint {
        method = POST,
        path = "/api/update/devices",
    }]
    pub async fn heartbeat_api_update_post_devices(
        rqctx: Arc<RequestContext<ServerContext>>,
    ) -> Result<HttpResponseOk<super::model::Device>, HttpError> {
        Ok(HttpResponseOk(super::model::Device {
            device_name: String::from("devicename"),
            last_beat: super::model::Beat {
                device_name: String::from("devicename"),
                timestamp: 0,
            },
            total_beats: 0,
            longest_missing_beat: 0,
        }))
    }

    #[endpoint {
        method = GET,
        path = "/api/info",
    }]
    pub async fn heartbeat_api_get_info(
        rqctx: Arc<RequestContext<ServerContext>>,
    ) -> Result<HttpResponseOk<super::model::Info>, HttpError> {
        Ok(HttpResponseOk(super::model::Info {
            last_seen: String::from("never"),
            time_difference: String::from("GMT+0"),
            missing_beat: String::from("never"),
            total_beats: String::from("0"),
        }))
    }

    #[endpoint {
        method = GET,
        path = "/api/stats",
    }]
    pub async fn heartbeat_api_get_stats(
        rqctx: Arc<RequestContext<ServerContext>>,
    ) -> Result<HttpResponseOk<super::model::Stats>, HttpError> {
        Ok(HttpResponseOk(super::model::Stats {
            last_beat_formatted: None,
            total_devices_formatted: None,
            total_visits_formatted: None,
            total_uptime_formatted: None,
            total_beats_formatted: None,
            total_visits: 0,
            total_uptime_milli: 0,
            total_beats: 0,
            longest_missing_beat: 0,
        }))
    }

    #[endpoint {
        method = GET,
        path = "/api/devices",
    }]
    pub async fn heartbeat_api_get_devices(
        rqctx: Arc<RequestContext<ServerContext>>,
    ) -> Result<HttpResponseOk<Vec<super::model::Device>>, HttpError> {
        let request = rqctx.request.lock().await;
        let context = rqctx.context();
        let auth = request.headers().get("Auth").unwrap();
        match context.is_authed(auth) {
            Ok(_) => todo!(),
            Err(e) => Err(HttpError::for_bad_request(
                Some(String::from("Unauthenticated")),
                e,
            )),
        }
    }
}
