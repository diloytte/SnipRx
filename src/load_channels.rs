use grammers_client::{Client, InvocationError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ChannelData {
    name: String,
    id: i64,
}

pub async fn load_channels_data(client: &Client) -> Result<Vec<ChannelData>, InvocationError> {
    let mut iter_dialogs = client.iter_dialogs();

    let dialogs_len = iter_dialogs.total().await.unwrap_or(0);

    let mut channels_data: Vec<ChannelData> = vec![];

    for _ in 0..dialogs_len {
        let next_dialog_option = iter_dialogs.next().await?;
        if let Some(next_dialog) = next_dialog_option {
            let chat = next_dialog.chat();
            match chat {
                grammers_client::types::Chat::Channel(channel) => {
                    let channel_name = channel.title();
                    let channel_id = channel.id();
                    channels_data.push(ChannelData {
                        name: channel_name.to_string(),
                        id: channel_id,
                    });
                }
                _ => {}
            }
        }
    }

    Ok(channels_data)
}
