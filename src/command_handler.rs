use crate::command::*;
use crate::context::Context;
use crate::response::*;
use femtopb::{EnumValue, Message};

pub trait CommandHandler {
    async fn handle(&self, id: u32, ctx: &Context) -> Response;
}

impl CommandHandler for SetLed<'_> {
    async fn handle(&self, id: u32, ctx: &Context) -> Response {
        let mut led = ctx.led.lock().await;
        if self.on {
            led.set_high();
        } else {
            led.set_low();
        }

        Response {
            id,
            status: EnumValue::Known(Status::Ok),
            ..Default::default()
        }
    }
}

impl CommandHandler for PowerControl<'_> {
    async fn handle(&self, id: u32, ctx: &Context) -> Response {
        let mut power = ctx.power.lock().await;
        if self.enable {
            power.set_high();
        } else {
            power.set_low();
        }

        Response {
            id,
            status: EnumValue::Known(Status::Ok),
            ..Default::default()
        }
    }
}

pub async fn dispatch_command(cmd: Command<'_>, ctx: &Context) -> Option<[u8; 64]> {
    let resp = match cmd.action {
        Some(command::Action::SetLed(ref led)) => led.handle(cmd.id, ctx).await,
        Some(command::Action::PowerControl(ref power)) => power.handle(cmd.id, ctx).await,
        _ => Response {
            id: cmd.id,
            status: EnumValue::Known(Status::Invalid),
            ..Default::default()
        },
    };

    let mut buf = [0; 64];
    if resp.encode(&mut &mut buf[..]).is_ok() {
        Some(buf)
    } else {
        None
    }
}
