mod load_channels;
mod load_required_env_variables;
mod save_json;
mod process_ignored_channels;
mod forward_message;

use std::{collections::HashMap, hash::Hash, pin::Pin, process::exit, vec};

use dotenv::dotenv;
use forward_message::forward_message;
use grammers_client::{Client, Config, InvocationError, SignInError, Update};
use grammers_session::Session;
use load_channels::{load_channels_and_additional_data, ChannelData};
use load_required_env_variables::load_required_env_variables;
use save_json::save_json_to_file;
// use process_ignored_channels::process_ignored_channels;
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

    // println!("Enter channel names to ignore. Split channel names using # sign.");
    // println!("Example:\n ChannelName1#ChannelName2#ChannelName3");

    // let mut channels_to_ignore = String::new();
    // std::io::stdin().read_line(&mut channels_to_ignore)?;

    // process_ignored_channels(channels_to_ignore);

    let session_data = client.session().save();
    fs::write(session_file, session_data).await?;

    println!("Connected to Telegram!");

    let channels_data_and_additional_data = load_channels_and_additional_data(&client).await?;

    let mut channels_map: HashMap<i64, i32> = HashMap::new();
    
    channels_data_and_additional_data.0.iter().for_each(|channel|{
        channels_map.insert(channel.id, 1);
    });

    let ignored_channel_ids:Vec<i64> = vec![
        2361478254,
        1667933245,
        1836088744,
        2241857744,
        2143300041
    ];

    for (id, value) in channels_map.iter_mut() {
        if ignored_channel_ids.contains(id) {
            *value = 0;
        }
    }
    let _ = save_json_to_file(&channels_data_and_additional_data.0, "./channels.json");

    let enable_loop = false;

    if !enable_loop {
        panic!("Loop not enabled.");
    }

    loop {
        match client.next_update().await {
            Ok(Update::NewMessage(message)) if !message.outgoing() => {
                let message_chat = message.chat();
                let chat_id = message_chat.id();
                    let is_channel_active_option = channels_map.get(&chat_id);
                    if let Some(is_channel_active) = is_channel_active_option {
                        if *is_channel_active == 1 {
                            let _ = forward_message(&client, &message, &channels_data_and_additional_data.1.trader_chat).await;
                        }
                    }
            }
            Err(e) => eprintln!("Error in listen_for_updates: {}", e),
            _ => {}
        }
    }
}