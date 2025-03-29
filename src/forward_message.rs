use grammers_client::{Client, InvocationError};
use grammers_client::types::Message;
use grammers_client::types::Chat;

pub async fn forward_message(client:&Client,message:&Message,forward_to:&Chat)->Result<(),InvocationError>{
    client.forward_messages(forward_to, &[message.id()], message.chat()).await?;

    let forward_to_chat_name = forward_to.name();

    let chat = message.chat();

    let mut channel_name: String = "UNNAMED_CHANNEL".to_string();

    match chat {
        Chat::Channel(channel) => {
            channel_name = channel.title().to_string();
        },
        _ => {}
    }
    
    let final_message = format!("Forwarded message from channel: {} to {} trader.",channel_name,forward_to_chat_name);

    println!("{}",final_message);

    Ok(())
}