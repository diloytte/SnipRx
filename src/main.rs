mod load_channels;
mod load_required_env_variables;
mod save_json;

use dotenv::dotenv;
use grammers_client::{Client, Config, SignInError};
use grammers_session::Session;
use load_channels::load_channels_data;
use load_required_env_variables::load_required_env_variables;
use save_json::save_json_to_file;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let env_variables = load_required_env_variables()?;

    let session_file = "session.session";
    let session = if let Ok(data) = fs::read(session_file).await {
        Session::load(&data)?
    } else {
        Session::new()
    };

    let client = Client::connect(Config {
        session,
        api_id: env_variables.api_id,
        api_hash: env_variables.api_hash,
        params: Default::default(),
    })
    .await?;

    if !client.is_authorized().await? {
        let token = client
            .request_login_code(&env_variables.phone_number)
            .await?;

        println!("Enter the OTP code:");
        let mut code = String::new();
        std::io::stdin().read_line(&mut code)?;
        let code = code.trim();

        match client.sign_in(&token, code).await {
            Ok(_) => println!("Logged in successfully!"),
            Err(SignInError::PasswordRequired(password_token)) => {
                client
                    .check_password(password_token, env_variables.password)
                    .await?;
            }
            Err(e) => return Err(e.into()),
        }
    }

    let session_data = client.session().save();
    fs::write(session_file, session_data).await?;

    println!("Connected to Telegram!");

    let channels_data = load_channels_data(&client).await?;

    save_json_to_file(&channels_data, "./channels.json")?;

    Ok(())
}
