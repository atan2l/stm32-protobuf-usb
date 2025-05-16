use crate::command::Command;
use crate::command_handler::dispatch_command;
use crate::comms::{USB_RX, USB_TX};
use crate::context::Context;
use femtopb::Message;

#[embassy_executor::task]
pub async fn command_dispatcher(ctx: &'static Context) {
    let rx = USB_RX.receiver();
    let tx = USB_TX.sender();

    loop {
        let pkt = rx.receive().await;
        if let Ok(cmd) = Command::decode(&pkt) {
            if let Some(reply) = dispatch_command(cmd, ctx).await {
                let _ = tx.send(reply).await;
            }
        }
    }
}
