pub async fn get_status(
    client: &reqwest::Client,
    access_token: &str,
    air_conditioner_id: &str,
) -> Result<String, reqwest::Error> {
    let response = client
        .get(super::constants::STATUS_URL)
        .header("Authorization", format!("Bearer {access_token}"))
        .query(&std::collections::HashMap::from([(
            "ACId",
            air_conditioner_id,
        )]))
        .send()
        .await?;

    response.text().await
}
