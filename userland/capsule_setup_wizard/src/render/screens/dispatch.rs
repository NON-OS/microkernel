use crate::server::step::{default_key, Outcome};
use crate::state::Context;

use super::{appearance, keyboard, local_software, mode, network, privacy, review, timezone};

pub fn draw(ctx: &Context) {
    match ctx.step {
        0 => keyboard::draw(ctx),
        1 => timezone::draw(ctx),
        2 => mode::draw(ctx),
        3 => network::draw(ctx),
        4 => privacy::draw(ctx),
        5 => appearance::draw(ctx),
        6 => local_software::draw(ctx),
        7 => review::draw(ctx),
        _ => crate::render::frame(ctx, b"Setup", b"", b"ENTER NEXT  ESC BACK"),
    }
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    match ctx.step {
        0 => keyboard::on_key(ctx, code),
        1 => timezone::on_key(ctx, code),
        2 => mode::on_key(ctx, code),
        3 => crate::network::on_key(ctx, code),
        4 => privacy::on_key(ctx, code),
        5 => appearance::on_key(ctx, code),
        6 => local_software::on_key(ctx, code),
        7 => review::on_key(ctx, code),
        _ => default_key(code),
    }
}
