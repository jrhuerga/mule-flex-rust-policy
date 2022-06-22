use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use log::{info, warn, trace};
use serde::{Default, Deserialize, Serialize};

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(HttpConfigHeaderRoot {
            secret: String::new()
        })
    });
}}

struct HttpConfigHeader {
    //header_content: String,
    secret: String
}

impl Context for HttpConfigHeader {}

impl HttpContext for HttpConfigHeader {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        info!("on_http_request_headers");
        //info!("self header content {}",self.header_content);
        info!("self header content {}",self.secret);
        Action::Continue
    }

    fn on_http_request_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        info!("on_http_request_body");
        //info!("self header content {}",self.header_content);
        info!("self header content {}",self.secret);
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        info!("on_http_response_headers");
        //info!("self header content {}",self.header_content);
        info!("self header content {}",self.secret);
        Action::Continue
    }

    fn on_http_response_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        info!("on_http_response_body");
        //info!("self header content {}",self.header_content);
        info!("self header content {}",self.secret);
        Action::Continue
    }
}

#[derive(Default, Serialize, Deserialize)]
struct PolicyConfig {
     #[serde(alias = "secret-value")]
    secret_value: String
}

struct HttpConfigHeaderRoot {
//    header_content: String,
    secret: String
}

impl Context for HttpConfigHeaderRoot {}

impl RootContext for HttpConfigHeaderRoot {
    fn on_configure(&mut self, _: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            let config = serde_json::from_slice(config_bytes.as_slice()).unwrap()
            self.secret = config.secret_value;
            //let strConfig = String::from_utf8(config_bytes).unwrap();
            //let c: Config = serde_json::from_str(&strConfig);
            //self.header_content = c.secret_value;

            //self.header_content = String::from_utf8(config_bytes).unwrap();
            info!("header_content {}",self.config.secret_value);
        }
        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpConfigHeader {
            secret: self.secret.clone(),
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}
