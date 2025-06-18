use anyhow::Error;
use async_channel::Sender;
use futures::StreamExt;
use gpui::App;
use lipl_display_common::{Command, Message};
use lipl_gatt_bluer::listen_stream;

pub fn init(cx: &mut App, sender: Sender<Message>) {
    gpui_tokio::Tokio::handle(cx).spawn(async move {
        let mut s = listen_stream()
            .await
            .inspect_err(|e| log::error!("Error:{e}"))
            .map_err(Error::from)?;

        sender
            .send(Message::Command(Command::Wait))
            .await
            .inspect_err(|e| log::error!("Error: {e}"))?;

        while let Some(message) = s.next().await {
            sender
                .send(message)
                .await
                .inspect_err(|e| log::error!("Error: {e}"))
                .map_err(Error::from)?;
        }
        Ok::<(), Error>(())
    });
}
