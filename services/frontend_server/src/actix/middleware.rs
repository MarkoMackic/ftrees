use std::future::{ready, Ready};
use std::sync::{Arc};
use std::rc::{Rc};
use futures_util::future::LocalBoxFuture;
use actix_web::{
    Error,
    HttpResponse,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready}
};
use crate::traits::{AccessHandler};
use crate::types::{AccessGrant};
use crate::handlers::check_perms::{CheckPerms};
use path_tree::{PathTree};


pub struct StaticFilesFirewall {
    cfg: Arc<serde_yaml::Value>
}

impl StaticFilesFirewall
{
    pub fn construct(val : Arc<serde_yaml::Value>) -> Self {
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

        sfm.construct(self.cfg.clone());

        ready(Ok(sfm))
    }
}

pub struct StaticFilesFirewallMiddleware<S> {
    service: S,
    router: PathTree<Rc<dyn AccessHandler>>
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
        return match self.router.find(&String::from(req.path())) {
            Some(res_tuple) => {
                let (handler, p) = res_tuple;

                let ag: AccessGrant = handler.handle(&p.params(), &req);

                Box::pin(ready(Ok(req.into_response(
                    HttpResponse::Ok().append_header(("Content-Type", "text/plain")).body(format!("{:?}", ag))
                ))))
            },
            None => {
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
            router: PathTree::new()
        }
    }

    pub fn construct(&mut self, cfg: Arc<serde_yaml::Value>) {
        if !cfg.as_mapping().unwrap().contains_key("routes") {
            return;
        }

        for route_mapping in cfg.get("routes").expect("routes").as_sequence().expect("routes_seq")
        {  
            let path = route_mapping.get("route").expect("route").get("path").expect("path");

            let rp = Rc::new(CheckPerms::new(route_mapping));

            let paths: Vec<&str> = match path {
                serde_yaml::value::Value::Sequence(val)=> (*val).iter().map(|a| a.as_str().expect("string")).collect(),
                serde_yaml::value::Value::String(val)=> vec!(val),
                _ => panic!("not ok")
            };

            for p in paths {
                self.router.insert(p, rp.clone());
            }
        }

    }
}

