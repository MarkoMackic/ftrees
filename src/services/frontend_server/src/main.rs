mod lib;

use std::env;
use std::fs::*;
use actix_files as af;
use actix_web::{web, App, HttpServer};
use lib::{api, middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cd = env::current_dir().expect("Can't read current dir!");
    
    let config = cd.join("config.yaml");

    let cfile = File::open(&config).expect("Error opening config file");

    let data = serde_yaml::from_reader(cfile).expect("Yaml parsing error");
    
    let data: &'static serde_yaml::Value = Box::leak(Box::new(data));

    let docroot = data.get("docroot").expect("Couldn't get docroot").as_str().expect("Can't convert docroot to string");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::StaticFilesFirewall::construct(data))
            .service(web::scope("/api").service(api::healthz))
            .service(af::Files::new("/", docroot).index_file("index.html"))
            .default_service(web::route().to(api::forbidden))
    }).bind(("0.0.0.0", 9090))?.run().await?;

    Ok(())
}
