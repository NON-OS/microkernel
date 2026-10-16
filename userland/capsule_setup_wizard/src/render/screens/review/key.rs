//! ENTER on the review screen applies the answers.

use crate::server::step::{default_key, Outcome, K_ENTER, K_ENTER_LF};
use crate::state::Context;

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if code == K_ENTER || code == K_ENTER_LF {
        /*
         * Answers the settings service refused, and a saved network dropped
         * to make room, are named before setup goes on, once: the second
         * Enter asks again and goes on either way.
         */
        super::super::commit::commit(ctx);
        if (ctx.unsaved != 0 || ctx.net.dropped.is_some()) && !ctx.unsaved_told {
            ctx.unsaved_told = true;
            return Outcome::Stay;
        }
        return Outcome::Advance;
    }
    default_key(code)
}
