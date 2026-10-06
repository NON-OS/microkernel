use nonos_libc::{mk_idle_ms, mk_service_lookup};

// The kernel registers driver.i2c_pci0 when it spawns the controller driver,
// which it does before this one, and drops it when that driver exits (no
// controller, or a controller that never came up). A single lookup can still
// race a spawn that has not finished, so it is retried inside a short window,
// a pause apart. The pauses are real sleeps: retrying with yields ran flat out
// on an idle machine and gave no slow registration any more time. Past the
// window the controller driver is gone, and so is the bus.
const LOOKUP_ATTEMPTS: u32 = 100;
const LOOKUP_PAUSE_MS: u64 = 20;

pub fn resolve() -> Option<(u32, u32)> {
    lookup_within(LOOKUP_ATTEMPTS, lookup_once, || {
        let _ = mk_idle_ms(LOOKUP_PAUSE_MS);
    })
}

fn lookup_once() -> Option<(u32, u32)> {
    let name = b"driver.i2c_pci0";
    let mut port = 0u32;
    let mut pid = 0u32;
    let r = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (r >= 0 && port != 0 && pid != 0).then_some((port, pid))
}

/// Up to `attempts` lookups, with a `pause` between two of them and none
/// after the last, so the wait is bounded by the attempts and the pauses.
fn lookup_within<T>(
    attempts: u32,
    mut lookup: impl FnMut() -> Option<T>,
    mut pause: impl FnMut(),
) -> Option<T> {
    for attempt in 0..attempts {
        if attempt > 0 {
            pause();
        }
        if let Some(found) = lookup() {
            return Some(found);
        }
    }
    None
}
