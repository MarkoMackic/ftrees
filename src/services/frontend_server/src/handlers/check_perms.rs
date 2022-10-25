use crate::types::{AccessGrant};
use crate::traits::{AccessHandler};
use matchit::Params;
use actix_web::dev::{ServiceRequest};
use serde_yaml::{Value};
use std::rc::Rc;

#[derive(Debug,Default)]
pub struct CheckPerms {
    route_cfg: Value
}

impl CheckPerms {
    pub fn new(route_cfg:&Value) -> Self
    {
        Self {
            route_cfg: serde_yaml::from_str(&serde_yaml::to_string(route_cfg).unwrap()).unwrap()
        }
    }
}


impl AccessHandler for CheckPerms {
    fn handle(&self, params:Params, req:&ServiceRequest) -> AccessGrant {
        println!("{:?}", req);
        AccessGrant::DENY
    }
}


