mod config;
mod toshiba_api;

use anyhow::Result;
use config::get_config;
use toshiba_api::login::LoginMessage;

use crate::toshiba_api::mapping::{parse_state_data, AirConditionerMode, PowerState};

#[tokio::main]
async fn main() -> Result<()> {
    let app_config = get_config()?;
    let http_client = reqwest::Client::new();
    let login_message = toshiba_api::login::login(&http_client, &app_config).await?;

    let login_details = match login_message {
        LoginMessage::InvalidUserNameOrPassword { message, .. } => {
            println!("Error: Invalid login. {}", message);
            std::process::exit(1);
        }
        LoginMessage::Success { res_obj, .. } => res_obj,
    };

    let mapping_result = toshiba_api::mapping::get_mappings(
        &http_client,
        &login_details.access_token,
        &login_details.consumer_id,
    )
    .await?;

    for group in mapping_result.iter() {
        for unit in group.ac_list.iter() {
            println!("{}", unit.name);
            let state_data = parse_state_data(&unit.ac_state_data).unwrap();
            println!(
                "\tPower: {}",
                match state_data.power_status {
                    PowerState::On => "On",
                    PowerState::Off => "Off",
                    _ => "Unknown",
                }
            );
            println!(
                "\tMode: {}",
                match state_data.mode {
                    AirConditionerMode::Auto => "Auto",
                    AirConditionerMode::Cool => "Cool",
                    AirConditionerMode::Dry => "Dry",
                    AirConditionerMode::Fan => "Fan",
                    AirConditionerMode::Heat => "Heat",
                    _ => "Unknown",
                }
            );
            println!("\tCurrent Temp: {}", state_data.indoor_temp);
            println!("\tTarget Temp: {}", state_data.target_temperature);
        }
    }

    // let first_ac_mapping = mapping_result.get(0).unwrap().ac_list.get(0).unwrap();

    // let status_result =
    //     toshiba_api::status::get_status(&client, &login_details.access_token, &first_ac_mapping.id)
    //         .await?;

    // println!("{status_result}");

    Ok(())
}
