use matchit::Params;
use actix_web::dev::{ServiceRequest};
use crate::types::{AccessGrant};
use serde_yaml::{Value};
use std::result::Result;

pub trait AccessHandler {
    fn handle(&self, params:Params, req:&ServiceRequest) -> AccessGrant;
    fn construct_handler(&mut self, route_cfg:&Value) -> Result<(), String>;
}