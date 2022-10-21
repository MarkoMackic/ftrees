use std::future::{ready, Ready};
use futures_util::future::LocalBoxFuture;
use actix_web::{
    Error,
    HttpResponse,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready}
};
use crate::traits::{AccessHandler};
use crate::types::{AccessGrant};
use crate::handlers::check_perms::{CheckPerms};

use matchit;


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

        sfm.construct(self.cfg);

        ready(Ok(sfm))
    }
}

pub struct StaticFilesFirewallMiddleware<S> {
    service: S,
    router: matchit::Router<Box<dyn AccessHandler>>
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
        let mut path = String::from(req.path());

       //let sval = format!("{:?} \n {} \n {}", req, path, m );

        let res = self.router.at(path.as_mut());

        return match res {
            Ok(boxed_handler) => {
                let ag: AccessGrant = boxed_handler.value.handle(boxed_handler.params, &req);

                Box::pin(ready(Ok(req.into_response(
                    HttpResponse::Ok().append_header(("Content-Type", "text/plain")).body(format!("{:?}", ag))
                ))))
            },
            Err(_) => {
                Box::pin(self.service.call(req))
            }
        }

        //let resp = format!("{:?}", res);
        //Box::pin(ready(Ok(req.into_response(HttpResponse::Ok().append_header(("Content-Type", "text/plain")).body(path)))))       
    }
}

impl<S> StaticFilesFirewallMiddleware<S> {
    fn default(svc: S) -> Self {
        StaticFilesFirewallMiddleware {
            service: svc,
            router: matchit::Router::new()
        }
    }

    pub fn construct(&mut self, cfg: &'static serde_yaml::Value) {
        if !cfg.as_mapping().unwrap().contains_key("routes") {
            return;
        }

        for route_mapping in cfg.get("routes").expect("routes").as_sequence().expect("routes_seq")
        {  
            let path_str = String::from(route_mapping.get("route").expect("route").get("path").expect("path").as_str().expect("path_str"));

            let mut checkPerms = CheckPerms::default();

            checkPerms.construct_handler(route_mapping).expect("handler construct");

            self.router.insert(path_str, Box::from(CheckPerms::default())).expect("well well");
        }

    }
}

