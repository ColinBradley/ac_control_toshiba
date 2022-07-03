use serde::Deserialize;

pub async fn get_mappings(
    client: &reqwest::Client,
    access_token: &str,
    consumer_id: &str,
) -> Result<Vec<GroupMap>, reqwest::Error> {
    let response = client
        .get(super::constants::MAPPING_URL)
        .header("Authorization", format!("Bearer {access_token}"))
        .query(&std::collections::HashMap::from([(
            "consumerId",
            consumer_id,
        )]))
        .send()
        .await?;

    let mappings: MappingsResponse = response.json().await?;

    Ok(mappings.res_obj)
}

#[derive(Deserialize)]
pub struct MappingsResponse {
    #[serde(rename = "ResObj")]
    res_obj: Vec<GroupMap>,
    #[serde(rename = "IsSuccess")]
    is_success: bool,
    #[serde(rename = "Message")]
    message: String,
}

#[derive(Deserialize)]
pub struct GroupMap {
    #[serde(rename = "GroupId")]
    pub group_id: String,
    #[serde(rename = "GroupName")]
    pub group_name: String,
    #[serde(rename = "ConsumerId")]
    pub consumer_id: String,
    #[serde(rename = "TimeZone")]
    pub time_zone: String,
    #[serde(rename = "ACList")]
    pub ac_list: Vec<AirConditionerMapping>,
}

#[derive(Deserialize)]
pub struct AirConditionerMapping {
    #[serde(rename = "Id")]
    pub id: String, //"2ba90997-30f8-4e18-a60f-114438ada0c5",
    #[serde(rename = "DeviceUniqueId")]
    pub device_unique_id: String, //"fde8e19f-4191-44b4-990c-bb91f5124be1",
    #[serde(rename = "Name")]
    pub name: String, //"Living room",
    #[serde(rename = "ACModelId")]
    pub acmodel_id: String, //"3",
    #[serde(rename = "Description")]
    pub description: String, //"AC_fde8e19f-4191-44b4-990c-bb91f5124be1",
    #[serde(rename = "CreatedDate")]
    pub created_date: String, //"6/14/2022 3:13:00 PM",
    #[serde(rename = "ACStateData")]
    pub ac_state_data: String, //"31411841316400101710fe0b00001002000000",
    #[serde(rename = "FirmwareUpgradeStatus")]
    pub firmware_upgrade_status: String, //"",
    #[serde(rename = "URL")]
    pub url: String, //"",
    #[serde(rename = "File")]
    pub file: String, //"",
    #[serde(rename = "MeritFeature")]
    pub merit_feature: String, //"2c02",
    #[serde(rename = "AdapterType")]
    pub adapter_type: String, //"0",
    #[serde(rename = "FirmwareVersion")]
    pub firmware_version: String, //"2.0.00",
    #[serde(rename = "FirmwareCode")]
    pub firmware_code: String, //"0001",
}

pub fn parse_state_data(value: &str) -> Result<StateData, std::num::ParseIntError> {
    let padded = [&value[..12], "0", &value[12..13], "0", &value[13..]].concat();

    Ok(StateData {
        power_status: match u8::from_str_radix(&padded[0..2], 16).unwrap() {
            0x30 => PowerState::On,
            0x31 => PowerState::Off,
            _ => PowerState::Unknown,
        },
        mode: match u8::from_str_radix(&padded[2..4], 16).unwrap() {
            0x41 => AirConditionerMode::Auto,
            0x42 => AirConditionerMode::Cool,
            0x43 => AirConditionerMode::Heat,
            0x44 => AirConditionerMode::Dry,
            0x45 => AirConditionerMode::Fan,
            _ => AirConditionerMode::Unknown,
        },
        target_temperature: u8::from_str_radix(&padded[4..6], 16).unwrap(),
        fan_mode: match u8::from_str_radix(&padded[6..8], 16).unwrap() {
            0x41 => FanMode::Auto,
            0x31 => FanMode::Quiet,
            0x32 => FanMode::Low,
            0x33 => FanMode::MediumLow,
            0x34 => FanMode::Medium,
            0x35 => FanMode::MediumHigh,
            0x36 => FanMode::High,
            0x00 => FanMode::None,
            _ => FanMode::Unknown,
        },
        swing_mode: u8::from_str_radix(&padded[8..10], 16).unwrap(),
        power_selection: u8::from_str_radix(&padded[10..12], 16).unwrap(),
        merit_a: u8::from_str_radix(&padded[12..14], 16).unwrap(),
        merit_b: u8::from_str_radix(&padded[14..16], 16).unwrap(),
        air_pure_ion: u8::from_str_radix(&padded[16..18], 16).unwrap(),
        indoor_temp: u8::from_str_radix(&padded[18..20], 16).unwrap(),
        outdoor_temp: u8::from_str_radix(&padded[20..22], 16).unwrap(),
        self_cleaning: u8::from_str_radix(&padded[30..32], 16).unwrap(),
    })
}

#[derive(Debug)]
pub struct StateData {
    pub power_status: PowerState,
    pub mode: AirConditionerMode,
    pub target_temperature: u8,
    pub fan_mode: FanMode,
    pub swing_mode: u8,
    pub power_selection: u8,
    pub merit_a: u8,
    pub merit_b: u8,
    pub air_pure_ion: u8,
    pub indoor_temp: u8,
    pub outdoor_temp: u8,
    pub self_cleaning: u8,
}

#[derive(Debug)]
pub enum PowerState {
    On,
    Off,
    Unknown,
}

#[derive(Debug)]
pub enum AirConditionerMode {
    Auto,
    Cool,
    Heat,
    Dry,
    Fan,
    Unknown,
}

#[derive(Debug)]
pub enum FanMode {
    Auto,
    Quiet,
    Low,
    MediumLow,
    Medium,
    MediumHigh,
    High,
    None,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let raw = "31411841316400101610fe0b00001002000000";
        let result = parse_state_data(raw);

        assert!(result.is_ok());
    }
}
