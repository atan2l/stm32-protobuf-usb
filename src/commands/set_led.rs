use femtopb::EnumValue;
use crate::command::SetLed;
use crate::command_handler::CommandHandler;
use crate::context::Context;
use crate::response::{Response, Status};

impl CommandHandler for SetLed<'_> {
    async fn handle(&self, id: u32, ctx: &Context) -> Response<'_> {
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