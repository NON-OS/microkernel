use nonos_keymap::POLICY_LAYOUTS;
use nonos_policy_proto::keyboard_layout_labels::KEYBOARD_LAYOUT_LABELS;

use nonos_policy_proto::Field;

use crate::clients::policy;
use crate::render::{self, widgets::rows};
use crate::server::step::{default_key, list_nav, Outcome};
use crate::state::Context;

/*
 * Only the layouts the PS/2 and USB keyboard drivers have tables for. The
 * policy lists more, and offering one of those would change a setting that
 * no key press ever reads.
 */
const COUNT: usize = POLICY_LAYOUTS.len();

/// The policy's index for the chosen row.
pub fn layout(sel: u8) -> u8 {
    POLICY_LAYOUTS.get(sel as usize).copied().unwrap_or(POLICY_LAYOUTS[0])
}

pub fn label(sel: u8) -> &'static [u8] {
    KEYBOARD_LAYOUT_LABELS.get(layout(sel) as usize).copied().unwrap_or(b"?")
}

pub fn draw(ctx: &Context) {
    render::frame(
        ctx,
        b"Keyboard layout",
        b"Up and Down or 1-6 to choose. Ctrl+Alt+Space cycles layouts at any time.",
        b"ENTER NEXT",
    );
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let buf = render::buffer(ctx);
    let mut items: [&[u8]; COUNT] = [b""; COUNT];
    for (i, item) in items.iter_mut().enumerate() {
        *item = label(i as u8);
    }
    let l = render::layout_of(ctx);
    rows::list(buf, spx, w, h, l.col_x, l.body_y, &items, ctx.kbd_sel as usize);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if let Some(o) = list_nav(&mut ctx.kbd_sel, COUNT as u8, code) {
        return o;
    }
    let o = default_key(code);
    let advancing = matches!(o, Outcome::Advance);
    let _ = super::keyboard_live::write_on_advance(
        advancing,
        ctx.policy_port,
        layout(ctx.kbd_sel),
        |port, value| policy::set_u8(port, Field::KeyboardLayout as u32, value),
    );
    o
}
