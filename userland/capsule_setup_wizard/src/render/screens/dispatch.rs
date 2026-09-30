use crate::server::order::{
    APPEARANCE, KEYBOARD, LOCAL_SOFTWARE, MODE, NAME, NETWORK, PRIVACY, QWEN, REVIEW, TIME_ZONE,
};
use crate::server::step::{default_key, Outcome};
use crate::state::Context;

use super::{
    appearance, keyboard, local_software, mode, name, network, privacy, qwen, review, timezone,
};

pub fn draw(ctx: &Context) {
    match ctx.step {
        KEYBOARD => keyboard::draw(ctx),
        NAME => name::draw(ctx),
        TIME_ZONE => timezone::draw(ctx),
        MODE => mode::draw(ctx),
        NETWORK => network::draw(ctx),
        PRIVACY => privacy::draw(ctx),
        APPEARANCE => appearance::draw(ctx),
        QWEN => qwen::draw(ctx),
        LOCAL_SOFTWARE => local_software::draw(ctx),
        REVIEW => review::draw(ctx),
        _ => crate::render::frame(ctx, b"Setup", b"", b"ENTER NEXT  ESC BACK"),
    }
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    match ctx.step {
        KEYBOARD => keyboard::on_key(ctx, code),
        NAME => name::on_key(ctx, code),
        TIME_ZONE => timezone::on_key(ctx, code),
        MODE => mode::on_key(ctx, code),
        NETWORK => crate::network::on_key(ctx, code),
        PRIVACY => privacy::on_key(ctx, code),
        APPEARANCE => appearance::on_key(ctx, code),
        QWEN => qwen::on_key(ctx, code),
        LOCAL_SOFTWARE => local_software::on_key(ctx, code),
        REVIEW => review::on_key(ctx, code),
        _ => default_key(code),
    }
}
