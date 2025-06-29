use femtopb::EnumValue;
use crate::command::PowerControl;
use crate::command_handler::CommandHandler;
use crate::context::Context;
use crate::response::{Response, Status};

impl CommandHandler for PowerControl<'_> {
    async fn handle(&self, id: u32, ctx: &Context) -> Response<'_> {
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