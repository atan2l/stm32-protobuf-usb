use crate::command::{command::*, Command};
use crate::command_handler::CommandHandler;
use crate::comms::{ProtoMessage, USB_RX, USB_TX};
use crate::context::Context;
use crate::response::{Response, Status};
use femtopb::{EnumValue, Message};

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

async fn dispatch_command(cmd: Command<'_>, ctx: &Context) -> Option<ProtoMessage> {
    let resp = match cmd.action {
        Some(Action::SetLed(ref led)) => led.handle(cmd.id, ctx).await,
        Some(Action::PowerControl(ref power)) => power.handle(cmd.id, ctx).await,
        Some(Action::PrintMessage(ref msg)) => msg.handle(cmd.id, ctx).await,
        _ => Response {
            id: cmd.id,
            status: EnumValue::Known(Status::Invalid),
            ..Default::default()
        },
    };

    let mut buf = ProtoMessage::new();
    let _ = buf.resize_default(buf.capacity());
    if resp.encode(&mut &mut buf[..]).is_ok() {
        buf.truncate(resp.encoded_len());
        Some(buf)
    } else {
        None
    }
}
