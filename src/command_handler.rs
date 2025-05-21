use crate::command::*;
use crate::comms::ProtoMessage;
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
            led.set_low();
        } else {
            led.set_high();
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

impl CommandHandler for PrintMessage<'_> {
    async fn handle(&self, id: u32, ctx: &Context) -> Response {
        use embedded_graphics::{
            mono_font::{ascii::FONT_6X10, MonoTextStyle},
            pixelcolor::BinaryColor,
            prelude::*,
            text::Text,
        };
        let display = &mut *ctx.display.lock().await;
        let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let _ = display.clear(BinaryColor::Off);
        if Text::new(self.message, Point::new(0, 0), style)
            .draw(display)
            .is_ok()
        {
            Response {
                id,
                status: EnumValue::Known(Status::Ok),
                ..Default::default()
            }
        } else {
            Response {
                id,
                status: EnumValue::Known(Status::Error),
                ..Default::default()
            }
        }
    }
}

pub async fn dispatch_command(cmd: Command<'_>, ctx: &Context) -> Option<ProtoMessage> {
    let resp = match cmd.action {
        Some(command::Action::SetLed(ref led)) => led.handle(cmd.id, ctx).await,
        Some(command::Action::PowerControl(ref power)) => power.handle(cmd.id, ctx).await,
        Some(command::Action::PrintMessage(ref msg)) => msg.handle(cmd.id, ctx).await,
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
