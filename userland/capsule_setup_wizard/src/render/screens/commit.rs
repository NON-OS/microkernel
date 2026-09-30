use nonos_policy_proto::Field;

use crate::clients::policy;
use crate::state::Context;

use super::{appearance, keyboard, mode};

/*
 * Persistent is written before anything is kept: vfs asks the policy store
 * for it on every persist and refuses one on an amnesic boot.
 */
pub fn commit(ctx: &mut Context) {
    let keep = mode::keeps(ctx);
    let p = ctx.policy_port;
    if p != 0 {
        let _ = policy::set_u8(p, Field::KeyboardLayout as u32, keyboard::layout(ctx.kbd_sel));
        let _ = policy::set_i8(p, Field::Timezone as u32, ctx.tz_off);
        let _ = policy::set_u8(p, Field::Wallpaper as u32, appearance::wallpaper(ctx.wall_sel));
        /* Empty leaves each unset: the system name, and no tier chosen. */
        let _ = policy::set_str(p, Field::Username as u32, ctx.name.typed());
        let _ = policy::set_str(p, Field::QwenTier as u32, ctx.qwen.chosen().unwrap_or(b""));
        let _ = policy::set_u8(p, Field::AppsOff as u32, ctx.apps_off);
        let _ = policy::set_bool(p, Field::Persistent as u32, keep);
    }
    crate::consent::apply(ctx.local_sel == 1, ctx.local_was, keep);
    if keep {
        crate::keep::save(ctx);
    }
    crate::network::keep(ctx);
}
