use crate::types::Error;
use dotenv_codegen::dotenv;
use serde::{de::DeserializeOwned, Serialize};

pub async fn request<B, T>(method: reqwest::Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let hostname = dotenv!("SERVER_FQDN");
    let port = dotenv!("PORT");
    let disable_https: bool = dotenv!("DISABLE_HTTPS").parse().unwrap_or(false);
    let url = format!(
        "{}://{}:{}/api{}",
        if disable_https { "http" } else { "https" },
        hostname,
        port,
        url
    );

    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let client: reqwest::Client = reqwest::ClientBuilder::new()
        .build()
        .expect("failed to build client");

    //fetch_credentials_include is for wasm32-unknown-unknown only
    let mut builder = client
        .request(method, &url)
        .fetch_credentials_include()
        .header("Content-Type", "application/json");

    if allow_body {
        builder = builder.json(&body);
    }

    log::info!("Sending request");
    let response = builder.send().await;
    log::info!("Got response");
    if let Ok(data) = response {
        if data.status().is_success() {
            let data: Result<T, _> = data.json::<T>().await;
            if let Ok(data) = data {
                log::info!("Response: {:?}", data);
                Ok(data)
            } else {
                log::error!("Failed to parse response: {:?}", data);
                Err(Error::DeserializeError)
            }
        } else {
            log::error!("Error: Response: {:?}", data);
            match data.status().as_u16() {
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                422 => Err(Error::UnprocessableEntity),
                500 => Err(Error::InternalServerError),
                502 => Err(Error::BadGateway),
                503 => Err(Error::ServiceUnavailable),
                504 => Err(Error::GatewayTimeout),
                _ => Err(Error::RequestError),
            }
        }
    } else {
        log::error!("Error: Response: {:?}", response);
        Err(Error::RequestError)
    }
}

pub async fn request_get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::GET, url, ()).await
}

pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::POST, url, body).await
}

pub async fn request_put<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::PUT, url, body).await
}

pub async fn request_delete<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::DELETE, url, ()).await
}
