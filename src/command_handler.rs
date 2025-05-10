use crate::command::*;
use crate::context::Context;
use crate::response::*;
use alloc::vec::Vec;
use prost::Message;

pub trait CommandHandler {
    async fn handle(&self, id: u32, ctx: &Context) -> Response;
}

impl CommandHandler for SetLed {
    async fn handle(&self, id: u32, ctx: &Context) -> Response {
        let mut led = ctx.led.lock().await;
        if self.on {
            led.set_high();
        } else {
            led.set_low();
        }

        Response {
            id,
            status: Status::Ok.into(),
        }
    }
}

impl CommandHandler for PowerControl {
    async fn handle(&self, id: u32, ctx: &Context) -> Response {
        todo!()
    }
}

pub async fn dispatch_command(cmd: Command, ctx: &Context) -> Option<Vec<u8>> {
    let resp = match cmd.action {
        Some(command::Action::SetLed(ref led)) => led.handle(cmd.id, ctx).await,
        Some(command::Action::PowerControl(ref power)) => power.handle(cmd.id, ctx).await,
        None => Response {
            id: cmd.id,
            status: Status::Invalid.into(),
        },
    };

    let mut buf = Vec::with_capacity(resp.encoded_len());
    if resp.encode(&mut buf).is_ok() {
        Some(buf)
    } else {
        None
    }
}
