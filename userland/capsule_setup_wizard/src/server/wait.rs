//! How long the input wait may block, and the work due when it returns.
//!
//! A wait of 0 blocks until a key arrives. Setup waits less while it still
//! needs the keyboard, while the disk may yet load consent from an earlier
//! boot, and while the network step is listing the networks it hears.

use crate::network;
use crate::state::Context;

/// How long to wait for input before asking for the keyboard again.
const GRAB_RETRY_MS: u64 = 100;

/// How often to ask whether the disk has loaded consent from an earlier boot.
const RESTORE_RETRY_MS: u64 = 500;

/* How often the Qwen step asks again whether the disk has loaded. */
const DISK_RETRY_MS: u64 = 250;

pub fn wait_ms(ctx: &Context, held: bool) -> u64 {
    if !held {
        return GRAB_RETRY_MS;
    }
    let restore = ctx.local_pending.then_some(RESTORE_RETRY_MS);
    let disk = ctx.qwen.pending.then_some(DISK_RETRY_MS);
    [restore, network::wait_ms(ctx), disk].into_iter().flatten().min().unwrap_or(0)
}

/// Do what is due. True when the screen should be drawn again.
pub fn tick(ctx: &mut Context) -> bool {
    let restored = ctx.local_pending && super::restore_poll::poll(ctx);
    let listed = network::poll(ctx);
    let disk = ctx.qwen.refresh();
    restored || listed || disk
}
