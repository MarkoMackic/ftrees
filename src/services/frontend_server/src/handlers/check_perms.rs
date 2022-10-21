use crate::types::{AccessGrant};
use crate::traits::{AccessHandler};
use matchit::Params;
use actix_web::dev::{ServiceRequest};
use serde_yaml::{Value};
use std::result::Result;

#[derive(Debug)]
pub struct CheckPerms {
    route_cfg: Value
}

impl CheckPerms {
    pub fn default() -> Self
    {
        Self {
            route_cfg: Value::default()
        }
    }
}


impl AccessHandler for CheckPerms {
    fn handle(&self, params:Params, req:&ServiceRequest) -> AccessGrant {
        println!("{:?}", req);
        AccessGrant::DENY
    }
    fn construct_handler(&mut self, route_cfg:&Value) -> Result<(), String>
    {
        self.route_cfg = serde_yaml::from_str(&serde_yaml::to_string(route_cfg).unwrap()).unwrap();
        Ok(())
    }
}


