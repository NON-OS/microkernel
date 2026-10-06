use nonos_policy_proto::Field;

use crate::clients::policy;
use crate::state::Context;

use super::unsaved::{
    note, ALL, APPS, COMPUTER, KEYBOARD, NAME, NETWORK, PERSISTENCE, QWEN, TIME_ZONE, WALLPAPER,
};
use super::{appearance, keyboard, mode};

/*
 * Persistent is written before anything is kept: vfs asks the policy store
 * for it on every persist and refuses one on an amnesic boot. An answer
 * the store refuses, or every one with no store to ask, is recorded in
 * `ctx.unsaved` for the review screen to name.
 */
pub fn commit(ctx: &mut Context) {
    let keep = mode::keeps(ctx);
    let p = ctx.policy_port;
    let mut u = if p == 0 { ALL } else { 0 };
    if p != 0 {
        note(
            &mut u,
            KEYBOARD,
            policy::set_u8(p, Field::KeyboardLayout as u32, keyboard::layout(ctx.kbd_sel)),
        );
        note(&mut u, TIME_ZONE, policy::set_i8(p, Field::Timezone as u32, ctx.tz_off));
        // The set first: the store takes no desktop wallpaper outside it.
        note(&mut u, WALLPAPER, policy::set_u64(p, Field::WallpapersKept as u32, ctx.walls_kept));
        note(
            &mut u,
            WALLPAPER,
            policy::set_u8(p, Field::Wallpaper as u32, appearance::wallpaper(ctx)),
        );
        /* Empty leaves each unset: the system name, and no tier chosen. */
        note(&mut u, NAME, policy::set_str(p, Field::Username as u32, ctx.name.typed()));
        let tier = ctx.qwen.chosen(keep).unwrap_or(b"");
        note(&mut u, QWEN, policy::set_str(p, Field::QwenTier as u32, tier));
        note(&mut u, APPS, policy::set_u8(p, Field::AppsOff as u32, ctx.apps_off));
        let route = crate::render::screens::route::route(ctx.route_sel);
        note(&mut u, NETWORK, policy::set_u8(p, Field::NetworkRoute as u32, route));
        /* The kernel takes no empty hostname, so empty sends nothing. */
        if !ctx.host.typed().is_empty() {
            note(&mut u, COMPUTER, policy::set_str(p, Field::Hostname as u32, ctx.host.typed()));
        }
        note(&mut u, PERSISTENCE, policy::set_bool(p, Field::Persistent as u32, keep));
    }
    ctx.unsaved = u;
    crate::consent::apply(ctx.local_sel == 1, ctx.local_was, keep);
    if keep {
        crate::keep::save(ctx);
    } else {
        crate::keep::stage(ctx);
    }
    crate::network::keep(ctx);
}
