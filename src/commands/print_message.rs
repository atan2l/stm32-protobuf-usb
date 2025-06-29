use femtopb::EnumValue;
use crate::command::PrintMessage;
use crate::command_handler::CommandHandler;
use crate::context::Context;
use crate::response::{Response, Status};

impl CommandHandler for PrintMessage<'_> {
    async fn handle(&self, id: u32, ctx: &Context) -> Response<'_> {
        use embedded_graphics::{
            mono_font::{ascii::FONT_6X10, MonoTextStyle},
            pixelcolor::BinaryColor,
            prelude::*,
            text::Text,
        };
        let display = &mut *ctx.display.lock().await;
        let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let _ = display.clear(BinaryColor::Off);
        if Text::new(self.message, Point::zero(), style)
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