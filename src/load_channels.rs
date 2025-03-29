use grammers_client::types::Chat;
use grammers_client::{Client, InvocationError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ChannelData {
    pub name: String,
    pub id: i64,
}

pub struct AdditionalData {
    pub trader_chat: Chat,
}

pub async fn load_channels_and_additional_data(
    client: &Client,
) -> Result<(Vec<ChannelData>, AdditionalData), InvocationError> {
    let mut iter_dialogs = client.iter_dialogs();

    let dialogs_len = iter_dialogs.total().await.unwrap_or(0);

    let mut channels_data: Vec<ChannelData> = vec![];

    let mut trader_chat: Option<Chat> = None;

    for _ in 0..dialogs_len {
        let next_dialog_option = iter_dialogs.next().await?;
        if let Some(next_dialog) = next_dialog_option {
            let chat = next_dialog.chat();
            if chat.id() == 6511860356 {
                trader_chat = Some(chat.clone());
            }
            if let grammers_client::types::Chat::Channel(channel) = chat {
                let channel_name = channel.title();
                let channel_id = channel.id();
                channels_data.push(ChannelData {
                    name: channel_name.to_string(),
                    id: channel_id,
                });
            }
        }
    }

    if trader_chat.is_none() {
        return Err(InvocationError::Dropped);
    }

    let additional_data = AdditionalData {
        trader_chat: trader_chat.unwrap(),
    };

    Ok((channels_data, additional_data))
}
