use crate::command::Command;
use crate::command_handler::dispatch_command;
use crate::comms::{Packet, USB_RX, USB_TX};
use crate::context::Context;
use prost::Message;

#[embassy_executor::task]
pub async fn command_dispatcher(ctx: &'static Context) {
    let rx = USB_RX.receiver();
    let tx = USB_TX.sender();

    loop {
        if let Ok(pkt) = rx.try_receive() {
            if let Ok(cmd) = Command::decode(&pkt[..]) {
                if let Some(reply) = dispatch_command(cmd, ctx).await {
                    if let Ok(pkt) = Packet::from_slice(&reply) {
                        let _ = tx.send(pkt).await;
                    }
                }
            }
        }
    }
}
