//! Keys on the network step when no passphrase is being typed.

use nonos_wifi_client::sealing_ready;

use crate::render::screens::mode;
use crate::server::step::{default_key, list_nav, Outcome, K_ENTER, K_ENTER_LF};
use crate::state::Context;

use super::join::{join, leave};
use super::poll::refresh;

const K_S: u32 = 0x73;
const K_UPPER_S: u32 = 0x53;
const K_R: u32 = 0x72;
const K_UPPER_R: u32 = 0x52;

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if ctx.net.typing {
        return super::typing::on_key(ctx, code);
    }
    if let Some(o) = start_typing(ctx, code) {
        return o;
    }
    let rows = 1 + ctx.net.count as u8;
    if let Some(o) = list_nav(&mut ctx.net.sel, rows, code) {
        return o;
    }
    match code {
        K_S | K_UPPER_S => refresh(ctx),
        K_R | K_UPPER_R if mode::keeps(ctx) => toggle_remember(ctx),
        K_ENTER | K_ENTER_LF => return enter(ctx),
        _ => return default_key(code),
    }
    Outcome::Stay
}

/*
 * "No network" leaves any network setup joined and moves on. A network
 * already joined moves on too; a secured one opens the passphrase editor
 * first, and an open one is joined at once.
 */
fn enter(ctx: &mut Context) -> Outcome {
    let Some(net) = ctx.net.selected() else {
        leave(ctx);
        return Outcome::Advance;
    };
    if ctx.net.joined.is_some_and(|j| j.ssid() == net.ssid()) {
        return Outcome::Advance;
    }
    if net.secured {
        ctx.net.wipe_pass();
        ctx.net.typing = true;
        ctx.net.result = None;
        return Outcome::Stay;
    }
    join(ctx, net);
    Outcome::Stay
}

/*
 * On a secured network not yet joined, a printable key is the passphrase's
 * first character: j, k, the digits and S would otherwise move the list or
 * look again, and what was typed would never show. The arrows still move.
 */
fn start_typing(ctx: &mut Context, code: u32) -> Option<Outcome> {
    if !(0x20..=0x7E).contains(&code) {
        return None;
    }
    let net = ctx.net.selected().filter(|n| n.secured)?;
    if ctx.net.joined.is_some_and(|j| j.ssid() == net.ssid()) {
        return None;
    }
    ctx.net.wipe_pass();
    ctx.net.typing = true;
    ctx.net.result = None;
    Some(super::typing::on_key(ctx, code))
}

/*
 * Turning it on first checks that a record can be sealed and kept on this
 * boot, so the step says now, rather than at the end, why it cannot.
 */
fn toggle_remember(ctx: &mut Context) {
    if ctx.net.remember {
        ctx.net.remember = false;
        return;
    }
    ctx.net.cannot_keep = sealing_ready().err();
    ctx.net.remember = ctx.net.cannot_keep.is_none();
}
