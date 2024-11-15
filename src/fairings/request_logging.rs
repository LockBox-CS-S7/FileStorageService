use std::net::IpAddr;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Route};
use rocket::http::Method;
use log::info;

pub struct RequestLogging;
#[rocket::async_trait]
impl Fairing for RequestLogging {
    fn info(&self) -> Info {
        Info {
            name: "request logging",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        let request_type = match request.method() {
            Method::Get => Some("GET "),
            Method::Post => Some("POST "),
            Method::Delete => Some("DELETE "),
            Method::Put => Some("PUT "),
            _ => None,
        };
        let req_route = request.route();
        let ip_address = request.client_ip();
        
        let request = ApiRequest::new(request_type, req_route, ip_address);
        info!("{}", request.get_log_message());
    }
}


#[derive(Debug, Clone, PartialEq)]
struct ApiRequest {
    request_type: Option<String>,
    requested_route: Option<String>,
    ip_address: Option<String>,
}
impl ApiRequest {
    fn new(request_type: Option<&str>, requested_route: Option<&Route>, ip_address: Option<IpAddr>) -> Self {
        let request_type = match request_type {
            Some(req_type) => Some(req_type.to_string()),
            None => None,
        };
        let requested_route = match requested_route {
            Some(route) => Some(route.to_string()),
            None => None,
        };
        let ip_address = match ip_address {
            Some(address) => Some(address.to_string()),
            None => None,
        };
        
        Self {
            request_type,
            requested_route,
            ip_address,
        }
    }
    
    fn get_log_message(&self) -> String {
        let mut message = String::from("received a request (request_type, route, ip_address): (");
        if let Some(t) = &self.request_type {
            message.push_str(t.as_str());
            message.push_str(", ");
        }
        if let Some(r) = &self.requested_route {
            message.push_str(r.as_str());
            message.push_str(", ");
        }
        if let Some(ip) = &self.ip_address {
            message.push_str(ip.as_str());
            message.push_str(")");
        }
        
        message
    }
}