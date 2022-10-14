use std::future::{ready, Ready};
use std::collections::HashMap;
use futures_util::future::LocalBoxFuture;
use actix_web::{
    Error,
    HttpResponse,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready}
};


pub struct StaticFilesFirewall {
    cfg: &'static serde_yaml::Value
}

impl StaticFilesFirewall
{
    pub fn construct(val : &'static serde_yaml::Value) -> Self {
        StaticFilesFirewall { cfg: val }
    }
}

impl<S> Transform<S, ServiceRequest> for StaticFilesFirewall
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = StaticFilesFirewallMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let mut sfm = StaticFilesFirewallMiddleware::default(service);

        sfm.construct_rules(self.cfg);

        ready(Ok(sfm))
    }
}

pub struct StaticFilesFirewallMiddleware<S> {
    service: S,
    paths: HashMap<String, &'static serde_yaml::Value>
}

impl<S> Service<ServiceRequest> for StaticFilesFirewallMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);


    fn call(&self, req: ServiceRequest) -> Self::Future 
    {
        let path = String::from(req.path());

        Box::pin(ready(Ok(req.into_response(HttpResponse::Ok().append_header(("Content-Type", "text/plain")).body(path)))))
    }
}

impl<S> StaticFilesFirewallMiddleware<S> {
    fn default(svc: S) -> Self {
        StaticFilesFirewallMiddleware {
            service: svc,
            paths: HashMap::new()
        }
    }

    pub fn construct_rules(&mut self, cfg: &'static serde_yaml::Value) {
        if !cfg.as_mapping().unwrap().contains_key("routes") {
            return;
        }

        for route_mapping in cfg.get("routes").expect("Routes as value").as_sequence().expect("Routes as sequence")
        {  
            self.paths.insert(String::from(route_mapping.get("route").unwrap().get("path").unwrap().as_str().unwrap()), route_mapping);
        }

    }
}

