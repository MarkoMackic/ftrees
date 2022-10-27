use actix_web::dev::{ServiceRequest};
use crate::types::{AccessGrant};

pub trait AccessHandler {
    fn handle(&self, params: &Vec<(&str, &str)>, req:&ServiceRequest) -> AccessGrant;
}