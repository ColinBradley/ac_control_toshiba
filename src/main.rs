mod config;
mod toshiba_api;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use anyhow::Result;
use config::get_config;
use serde::Serialize;
use tokio::sync::Mutex;
use toshiba_api::{login::LoginMessage, mapping::StateData};

use crate::toshiba_api::mapping::parse_state_data;

pub struct ServerState {
    config: config::Config,
    http_client: reqwest::Client,
    login_details: Mutex<Option<toshiba_api::login::LoginSuccessResponse>>,
}

#[derive(Serialize)]
struct AcUnitResult {
    name: String,
    state: StateData,
}

async fn get_units(req: HttpRequest) -> impl Responder {
    let server_state = req.app_data::<web::Data<ServerState>>().unwrap();

    let login_message = toshiba_api::login::login(&server_state.http_client, &server_state.config)
        .await
        .unwrap();

    let mut login_guard = server_state.login_details.lock().await;

    if login_guard.is_none() {
        let response = match login_message {
            LoginMessage::InvalidUserNameOrPassword { message, .. } => {
                println!("Error: Invalid login. {}", message);
                std::process::exit(1);
            }
            LoginMessage::Success { res_obj, .. } => res_obj,
        };

        *login_guard = Some(response);
    }

    let login = login_guard.as_ref().unwrap();

    // Take clones of the login details that we need, so that we can drop the guard sooner (and not keep it active during get_mappings)
    // .. Not really sure if that's necessary?
    let access_token = login.access_token.clone();
    let consumer_id = login.consumer_id.clone();

    let mut mapping_result =
        toshiba_api::mapping::get_mappings(&server_state.http_client, &access_token, &consumer_id)
            .await
            .unwrap();

    web::Json(
        mapping_result
            .remove(0)
            .ac_list
            .iter()
            .map(|unit| AcUnitResult {
                name: unit.name.clone(),
                state: parse_state_data(&unit.ac_state_data).unwrap(),
            })
            .collect::<Vec<AcUnitResult>>(),
    )
}

#[actix_web::main]
async fn main() -> Result<()> {
    println!("Starting server: http://localhost:8080/index.html");

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Compress::default())
            .app_data(web::Data::new(ServerState {
                config: get_config().unwrap(),
                http_client: reqwest::Client::new(),
                login_details: Mutex::from(None),
            }))
            .route("/api/units", web::get().to(get_units))
            .service(actix_files::Files::new("/", "./www"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
