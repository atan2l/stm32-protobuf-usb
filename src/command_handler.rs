use crate::context::Context;
use crate::response::Response;

pub trait CommandHandler {
    async fn handle(&self, id: u32, ctx: &Context) -> Response<'_>;
}
