use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
    time::Instant,
};

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// AppState Struct, holding the "Db"
struct AppState {
    map: Mutex<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
    user: String,
}

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(state: web::Data<Arc<AppState>>, data: String) -> HttpResponse {
    info!("In Login service");

    let login_data: LoginData = serde_json::from_str(&data).unwrap();
    if login_data.password == "test" {
        if let Ok(mut map) = state.map.lock() {
            let token = Uuid::new_v4().to_string();
            map.insert(token.clone(), login_data.username.clone());

            return HttpResponse::Ok().json(Token {
                token,
                user: login_data.username,
            });
        }
        HttpResponse::InternalServerError().body("")
    } else {
        HttpResponse::Forbidden().body("")
    }
}

#[get("/check/{token}")]
async fn check_token(
    state: web::Data<Arc<AppState>>,
    web::Path(token): web::Path<String>,
) -> HttpResponse {
    info!("In check service");
    info!("Checking {} token", token);
    if let Ok(map) = state.map.lock() {
        let token_option = map.get(&token);

        if let Some(saved_token) = token_option {
            return HttpResponse::Ok().body(saved_token);
        } else {
            return HttpResponse::NotFound().body("");
        }
    }
    HttpResponse::InternalServerError().body("")
}

#[get("/")]
async fn hello() -> HttpResponse {
    info!("In Hello service");

    HttpResponse::Ok().body("Hello, World!")
}

// Declaring the main methode as the actix-web main
#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    // We build the AppState
    let app_state = Arc::new(AppState {
        map: Mutex::new(HashMap::new()),
    });

    // Server creation
    let server = HttpServer::new(move || {
        // We build the CORS Rules
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);
        // We create the App
        App::new()
            // With the app_state
            .data(app_state.clone())
            // With the CORS filter
            .wrap(cors)
            // With the WS
            .service(hello)
            .service(web::scope("/token").service(login).service(check_token))
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
        Ok(_) => println!(".env file successfuly loaded"),
        // In case of error, log and panic
        Err(error) => {
            println!("An error occured while loading the .env file: {}", error);
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
