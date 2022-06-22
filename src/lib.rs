use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use log::{info, warn, trace};

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(HttpConfigHeaderRoot {
            header_content: String::new(),
        })
    });
}}

struct HttpConfigHeader {
    header_content: String,
}

impl Context for HttpConfigHeader {}

impl HttpContext for HttpConfigHeader {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        trace!("Log 01a ");
        info!("Log 01b ");
        warn!("Log 01c ");
        Action::Continue
    }

    fn on_http_request_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        trace!("Log 02a ");
        info!("Log 02b ");
        warn!("Log 02c ");
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        trace!("Log 03a ");
        info!("Log 03b ");
        warn!("Log 03c ");
        Action::Continue
    }

    fn on_http_response_body(&mut self, _body_size: usize, _end_of_stream: bool) -> Action {
        trace!("Log 04a ");
        info!("Log 04b ");
        warn!("Log 04c ");
        Action::Continue
    }
}

struct HttpConfigHeaderRoot {
    header_content: String,
}

impl Context for HttpConfigHeaderRoot {}

impl RootContext for HttpConfigHeaderRoot {
    fn on_configure(&mut self, _: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            self.header_content = String::from_utf8(config_bytes).unwrap()
        }
        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpConfigHeader {
            header_content: self.header_content.clone(),
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}
