//! Typing a passphrase: printable keys go into it, so j, k and digits are
//! characters here rather than list moves. It is shown masked, never written
//! to the console, and wiped on Escape or once a join fails.

use crate::server::step::Outcome;
use crate::state::Context;
use crate::text::{key, Typed};

use super::join::join;

/// WPA2 takes 8 to 63 characters, or 64 hex digits of key.
const PASS_MIN: usize = 8;
const K_TAB: u32 = 0x09;

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    let n = &mut ctx.net;
    // Tab shows what was typed, or hides it again.
    if code == K_TAB {
        n.show = !n.show;
        return Outcome::Stay;
    }
    match key(&mut n.pass, &mut n.pass_len, code, |_, _| Ok::<(), ()>(())) {
        Typed::Esc => n.wipe_pass(),
        Typed::Enter if n.pass_len >= PASS_MIN => {
            if let Some(net) = n.selected() {
                n.typing = false;
                join(ctx, net);
            }
        }
        _ => {}
    }
    Outcome::Stay
}
