// NONOS Operating System (AGPL-3.0-or-later)

//! A wait that never yields the CPU, for the one place Linux keeps the CPU
//! and the device busy on purpose: `iwl_pcie_spin_for_iml` (v6.12
//! pcie/trans-gen2.c). An integrated AX210-family part's boot ROM does not
//! set its latency tolerance right, and the host may not set it for it
//! (`iwl_pcie_set_ltr`); Linux's workaround is to read the image-loader
//! cause back to back, CSR_LTR_LAST_MSG between reads, with no sleep, until
//! the cause is set or 100 ms pass. A sleep between reads lets the CPU and
//! the link go idle, which is what the loop is there to prevent.

/// Re-check `ready` back to back until it holds, `now` (in milliseconds)
/// has moved `ms` past its first reading, or `cap` checks were made, so the
/// wait ends even on a clock that never moves. Nothing in it sleeps.
pub fn spin_until(
    now: &mut dyn FnMut() -> u64,
    ms: u32,
    cap: u32,
    ready: &mut dyn FnMut() -> bool,
) -> bool {
    let end = now().saturating_add(u64::from(ms));
    for _ in 0..cap {
        if ready() {
            return true;
        }
        if now() >= end {
            return false;
        }
    }
    false
}
