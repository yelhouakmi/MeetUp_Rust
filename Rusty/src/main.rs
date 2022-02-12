use std::{env, io::Result, time::Instant};

use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use log::{error, info, warn};

#[get("/")]
async fn hello() -> HttpResponse {
    info!("In Hello service");
    HttpResponse::Ok().body("Hello, World!")
}

// Declaring the main methode as the actix-web main
#[actix_web::main]
async fn main() -> Result<()> {
    // Initialising the start time
    let start = Instant::now();

    // Loading the ".env" properties file
    load_dot_env();

    // Loading the log4rs configuration
    let log_file = if let Ok(log_file) = env::var("log4rs.file") {
        log_file
    } else {
        let default_log_file = "log4rs.yaml".into();
        warn!(
            "No log4rs configuration file found in the env, using the default {:?}",
            default_log_file
        );
        default_log_file
    };
    load_log4rs_file(&log_file);

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);

        App::new().wrap(cors).service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run();
    println!("Lancement fini {:?}", start.elapsed());

    server.await
}

fn load_dot_env() {
    // In a real app, we should not care about the result
    // DotEnv replace the "env" variable
    // Error on DotEnv -> use the env. Which is the default on Prod.
    match dotenv() {
        // If everything is ok, log and continue
        Ok(_) => info!(".env file successfuly loaded"),
        // In case of error, log and panic
        Err(error) => {
            error!("An error occured while loading the .env file: {}", error);
            panic!("Error on .env file loading");
        }
    }
}

fn load_log4rs_file(log_file: &str) {
    match log4rs::init_file(log_file, Default::default()) {
        Ok(_) => info!("Log4rs configuration loaded"),
        Err(error) => {
            error!("Error while loading the log4rs configuration: {}", error);
            panic!("Error loading log4rs config");
        }
    }
}
