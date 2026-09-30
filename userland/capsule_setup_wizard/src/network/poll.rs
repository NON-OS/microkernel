//! Keeping the list current while the step is on screen.
//!
//! The driver scans in the background and answers from what it has heard, so
//! asking again every couple of seconds fills the list as networks are heard,
//! and shows the driver's bring-up stage until it is ready.

use nonos_libc::mk_time_millis;
use nonos_wifi_client::{find, DriverStage};

use crate::server::step::NETWORK_STEP;
use crate::state::Context;

const POLL_MS: i64 = 2_000;

/// How long the input wait may block before the list is due, or `None` when
/// the step is not refreshing (another step, typing, or already joined).
pub fn wait_ms(ctx: &Context) -> Option<u64> {
    if !refreshing(ctx) {
        return None;
    }
    let due = ctx.net.polled_ms + POLL_MS - mk_time_millis();
    Some(due.clamp(1, POLL_MS) as u64)
}

/// Refresh when due. True when the screen should be drawn again.
pub fn poll(ctx: &mut Context) -> bool {
    if !refreshing(ctx) || mk_time_millis() - ctx.net.polled_ms < POLL_MS {
        return false;
    }
    refresh(ctx);
    true
}

fn refreshing(ctx: &Context) -> bool {
    ctx.step == NETWORK_STEP && !ctx.net.typing && ctx.net.joined.is_none()
}

/// Find the driver, read its stage and, once it is ready, the networks.
pub fn refresh(ctx: &mut Context) {
    let n = &mut ctx.net;
    n.polled_ms = mk_time_millis();
    n.driver = find();
    n.stage = n.driver.and_then(|d| d.stage());
    let (Some(d), Some(DriverStage::Ready)) = (n.driver, n.stage) else {
        n.count = 0;
        n.sel = 0;
        return;
    };
    let keep = n.selected();
    let (count, _, _) = d.scan(&mut n.nets);
    n.nets[..count].sort_unstable_by(|a, b| b.signal.cmp(&a.signal));
    n.count = count;
    /*
     * Keep the highlight on the same network when the list reorders.
     */
    let at = keep.and_then(|k| n.nets[..count].iter().position(|x| x.ssid() == k.ssid()));
    n.sel = at.map_or(0, |i| i as u8 + 1);
}
