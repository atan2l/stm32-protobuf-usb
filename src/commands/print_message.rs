use crate::command::PrintMessage;
use crate::command_handler::CommandHandler;
use crate::context::Context;
use crate::response::{Response, Status};
use embedded_graphics::text::Baseline;
use femtopb::EnumValue;

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
        if display.clear(BinaryColor::Off).is_ok()
            && Text::with_baseline(self.message, Point::zero(), style, Baseline::Top)
                .draw(display)
                .is_ok()
            && display.flush().await.is_ok()
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
