use serde;
use serde::Deserialize;

use crate::config::Config;

pub async fn login(
    client: &reqwest::Client,
    config: &Config,
) -> Result<LoginMessage, reqwest::Error> {
    let response = client
        .post(super::constants::LOGIN_URL)
        .json(&std::collections::HashMap::from([
            ("Username", &config.username),
            ("Password", &config.password),
        ]))
        .send()
        .await?;

    response.json().await
}

#[derive(Deserialize)]
#[serde(tag = "StatusCode")]
pub enum LoginMessage {
    #[serde(rename = "InvalidUserNameorPassword")]
    InvalidUserNameOrPassword {
        #[serde(rename = "ResObj")]
        res_obj: LoginErrorResponse,
        #[serde(rename = "IsSuccess")]
        is_success: bool,
        #[serde(rename = "Message")]
        message: String,
    },
    Success {
        #[serde(rename = "ResObj")]
        res_obj: LoginSuccessResponse,
        #[serde(rename = "IsSuccess")]
        is_success: bool,
        #[serde(rename = "Message")]
        message: String,
    },
}

#[derive(Deserialize)]
pub struct LoginErrorResponse {
    pub error: String,
}

#[derive(Deserialize)]
pub struct LoginSuccessResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    #[serde(rename = "consumerId")]
    pub consumer_id: String,
    #[serde(rename = "countryId")]
    pub country_id: u32,
    #[serde(rename = "consumerMasterId")]
    pub consumer_master_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_parse() {
        let value = r#"{"ResObj":{"error":"invalid_grant"},"IsSuccess":false,"Message":"Invalid UserName or Password","StatusCode":"InvalidUserNameorPassword"}"#;
        let result = serde_json::from_str::<LoginMessage>(value);
        match result {
            Ok(LoginMessage::InvalidUserNameOrPassword {
                res_obj, message, ..
            }) => {
                assert_eq!(res_obj.error, "invalid_grant");
                assert_eq!(message, "Invalid UserName or Password");
            }
            _ => {
                panic!("Failed to parse");
            }
        }
    }

    #[test]
    fn test_success_parse() {
        let value = r#"{"ResObj":{"access_token":"dave","token_type":"bearer","expires_in":1576799999,"consumerId":"steve","countryId":123,"consumerMasterId":"fred"},"IsSuccess":true,"Message":"Success","StatusCode":"Success"}"#;
        let result = serde_json::from_str::<LoginMessage>(value);
        match result {
            Ok(LoginMessage::Success {
                res_obj, message, ..
            }) => {
                assert_eq!(res_obj.access_token, "dave");
                assert_eq!(res_obj.expires_in, 1576799999);
                assert_eq!(message, "Success");
            }
            _ => {
                panic!("Failed to parse");
            }
        }
    }
}
