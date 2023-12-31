pub mod authentication {
    tonic::include_proto!("authentication");
}

use authentication::{auth_client::AuthClient, SignInRequest, SignOutRequest, SignUpRequest};

use crate::authentication::{SignInResponse, SignOutResponse, SignUpResponse, StatusCode};

use tokio::time::{sleep, Duration};
use tonic::Request;
use uuid::Uuid;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // AUTH_SERVICE_HOST_NAME will be set to 'auth' when running the health check service in Docker
    // ::0 is required for Docker to work:
    // https://stackoverflow.com/questions/59179831/docker-app-server-ip-address-127-0-0-1-difference-of-0-0-0-0-ip
    let auth_hostname = env::var("AUTH_SERVICE_HOST_NAME").unwrap_or("[::0]".to_owned());

    // Establish connection when auth service
    let mut client = AuthClient::connect(format!("http://{}:50051", auth_hostname)).await?;

    loop {
        let username: String = Uuid::new_v4().into();
        let password: String = Uuid::new_v4().into();

        let response: SignUpResponse = client
            .sign_up(Request::new(SignUpRequest {
                username: username.clone(),
                password: password.clone(),
            }))
            .await?
            .into_inner();

        println!(
            "SIGN UP RESPONSE STATUS: {:?}",
            StatusCode::from_i32(response.status_code)
        );

        // ---------------------------------------------

        let response: SignInResponse = client
            .sign_in(Request::new(SignInRequest { username, password }))
            .await?
            .into_inner();

        println!(
            "SIGN IN RESPONSE STATUS: {:?}",
            StatusCode::from_i32(response.status_code)
        );

        // ---------------------------------------------

        let response: SignOutResponse = client
            .sign_out(Request::new(SignOutRequest {
                session_token: response.session_token,
            }))
            .await?
            .into_inner();

        println!(
            "SIGN OUT RESPONSE STATUS: {:?}",
            StatusCode::from_i32(response.status_code)
        );

        println!("--------------------------------------",);

        sleep(Duration::from_secs(3)).await;
    }
}
