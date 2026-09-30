//! Typing a passphrase: printable keys go into it, so j, k and digits are
//! characters here rather than list moves. It is shown masked, never written
//! to the console, and wiped on Escape or once a join fails.

use nonos_wifi_client::PASS_MAX;

use crate::server::step::{Outcome, K_ENTER, K_ENTER_LF, K_ESC};
use crate::state::Context;

use super::join::join;

const K_BACKSPACE: u32 = 0x08;
const K_DELETE: u32 = 0x7F;
/// WPA2 takes 8 to 63 characters, or 64 hex digits of key.
const PASS_MIN: usize = 8;

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    let n = &mut ctx.net;
    match code {
        K_ESC => n.wipe_pass(),
        K_BACKSPACE | K_DELETE if n.pass_len > 0 => {
            n.pass_len -= 1;
            n.pass[n.pass_len] = 0;
        }
        0x20..=0x7E if n.pass_len < PASS_MAX => {
            n.pass[n.pass_len] = code as u8;
            n.pass_len += 1;
        }
        K_ENTER | K_ENTER_LF if n.pass_len >= PASS_MIN => {
            if let Some(net) = n.selected() {
                n.typing = false;
                join(ctx, net);
            }
        }
        _ => {}
    }
    Outcome::Stay
}
