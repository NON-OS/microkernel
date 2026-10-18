// NONOS Operating System (AGPL-3.0-or-later)
//! A full-speed hub on root port 1, addressed here and brought up by the
//! hub class's own `attach`, then walked until
//! its ports are brought up (hub_seen.rs binds).

use nonos_usb_hub::{attach, scan_hubs, Classes, Host, Hub, Place};

use crate::hub_lane::Lane;
use crate::hub_seen::{hub_dev, Seen, HUB_CONFIG};
use crate::model::hub::HubModel;
use crate::model::{addressed, rig, Cx, Shared};

/// The hub with `model`'s ports on root port 1, walked.
pub fn tree(model: HubModel) -> (Cx, Shared, Vec<Hub>, Seen) {
    let (mut cx, w) = rig([Some(hub_dev(model)), None]);
    let slot = addressed(&mut cx, 1);
    let mut lane = Lane { cx: &mut cx, lines: vec![] };
    let mut hubs = vec![attach(&mut lane, slot, Place::root(1, 1), &HUB_CONFIG).expect("hub up")];
    let mut seen = Seen::default();
    settle(&mut lane, &mut hubs, &mut seen);
    (cx, w, hubs, seen)
}

/// A pass over `hubs`, then the ports mid bring-up looked at again every
/// 10 ms, as driver.usb_hub0 looks at them, until none is.
pub fn settle<H: Host>(h: &mut H, hubs: &mut Vec<Hub>, c: &mut impl Classes<H>) {
    let mut busy = scan_hubs(h, hubs, c, false);
    for _ in 0..200 {
        if !busy {
            break;
        }
        h.idle_ms(10);
        busy = scan_hubs(h, hubs, c, true);
    }
}
