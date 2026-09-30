use crate::server::step::{default_key, Outcome};
use crate::state::Context;

use super::{appearance, keyboard, local_software, mode, privacy, review, timezone};

pub fn draw(ctx: &Context) {
    match ctx.step {
        0 => keyboard::draw(ctx),
        1 => timezone::draw(ctx),
        2 => mode::draw(ctx),
        3 => privacy::draw(ctx),
        4 => appearance::draw(ctx),
        5 => local_software::draw(ctx),
        6 => review::draw(ctx),
        _ => crate::render::frame(ctx, b"Setup", b"", b"ENTER NEXT  ESC BACK"),
    }
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    match ctx.step {
        0 => keyboard::on_key(ctx, code),
        1 => timezone::on_key(ctx, code),
        2 => mode::on_key(ctx, code),
        3 => privacy::on_key(ctx, code),
        4 => appearance::on_key(ctx, code),
        5 => local_software::on_key(ctx, code),
        6 => review::on_key(ctx, code),
        _ => default_key(code),
    }
}
