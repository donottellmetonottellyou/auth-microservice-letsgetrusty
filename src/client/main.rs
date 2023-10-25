pub mod authentication {
    tonic::include_proto!("authentication");
}

use authentication::{auth_client::AuthClient, SignInRequest, SignOutRequest, SignUpRequest};

use crate::authentication::{SignInResponse, SignOutResponse, SignUpResponse};

use clap::{Parser, Subcommand};
use tonic::{transport::Channel, Request};

use std::env;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
enum Commands {
    SignIn {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    SignUp {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    SignOut {
        #[arg(short, long)]
        session_token: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // AUTH_SERVICE_IP can be set to your droplet's ip address once your app is deployed
    let auth_ip = env::var("AUTH_SERVICE_IP").unwrap_or("[::0]".to_owned());
    let mut client: AuthClient<Channel> =
        AuthClient::connect(format!("http://{auth_ip}:50051")).await?;

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::SignIn { username, password }) => {
            let response: SignInResponse = client
                .sign_in(Request::new(SignInRequest {
                    username: username.clone(),
                    password: password.clone(),
                }))
                .await?
                .into_inner();

            println!("{response:?}");
        }
        Some(Commands::SignUp { username, password }) => {
            let response: SignUpResponse = client
                .sign_up(Request::new(SignUpRequest {
                    username: username.clone(),
                    password: password.clone(),
                }))
                .await?
                .into_inner();

            println!("{response:?}");
        }
        Some(Commands::SignOut { session_token }) => {
            let response: SignOutResponse = client
                .sign_out(Request::new(SignOutRequest {
                    session_token: session_token.clone(),
                }))
                .await?
                .into_inner();

            println!("{response:?}");
        }
        None => {}
    }

    Ok(())
}
