use matchit::Params;
use actix_web::dev::{ServiceRequest};
use crate::types::{AccessGrant};

pub trait AccessHandler {
    fn handle(&self, params:Params, req:&ServiceRequest) -> AccessGrant;
}