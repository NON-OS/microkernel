// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Bus Master at claim: the engines an owner that died without its
//! teardown left armed are stopped, and seen to be, before bus mastering
//! goes on. A reader that cannot be seen to stop them, or a command write
//! the broker refuses, never masters the bus, and the driver still lets
//! the reader go with the release last (see also stop_seen_tests.rs).

use nonos_libc::{bus_master, events, live_grants, refuse_bus_master, set_devices, violations};

use super::rig::{knobs, rts5227, taken};
use crate::error::RtsxError;
use crate::model::{left_armed, Knobs};
use crate::setup::{find, run};

/// Memory decode, Bus Master and the interrupt pin off.
const MASTER: &str = "pci 0x4=0x0406";

fn first(ev: &[String], what: &str) -> usize {
    ev.iter().position(|e| e == what).unwrap_or_else(|| panic!("no {what}: {ev:?}"))
}

#[test]
fn a_dead_owners_engines_are_seen_stopped_before_bus_mastering_goes_on() {
    left_armed();
    let _drv = taken();
    let ev = events();
    let on = first(&ev, MASTER);
    assert!(first(&ev, "stop cmd") < on && first(&ev, "stop dma") < on, "{ev:?}");
    let seen = "bus master on: cmd armed false stopped true; dma armed false stopped true";
    assert_eq!(ev.get(on + 1).map(String::as_str), Some(seen), "{ev:?}");
    assert_eq!(violations(), 0, "{ev:?}");
}

/// `run` refused with `want`, no Bus Master taken, the release last: its events.
pub(super) fn refused(want: RtsxError) -> Vec<String> {
    set_devices(vec![rts5227()]);
    let found = find().expect("listed");
    assert_eq!(run(&found).err(), Some(want));
    let ev = events();
    assert!(ev.iter().all(|e| e != MASTER) && !bus_master(), "{ev:?}");
    let release = ev.iter().rposition(|e| e.starts_with("release"));
    assert_eq!((release, live_grants()), (Some(ev.len() - 1), 0), "{ev:?}");
    ev
}

#[test]
fn a_reader_not_seen_to_stop_its_engines_never_masters_the_bus() {
    knobs(Knobs { gone: true, ..Knobs::default() });
    refused(RtsxError::Gone);
}

#[test]
fn a_refused_bus_master_write_quiesces_and_releases_the_reader() {
    refuse_bus_master();
    let ev = refused(RtsxError::PciCommand);
    let no = first(&ev, "pci 0x4=0x0406 refused");
    let after = &ev[no..];
    let quiet = ["stop cmd", "stop dma", "bier 0x0", "pci 0x4=0x0402"].map(|e| first(after, e));
    assert!(quiet.windows(2).all(|w| w[0] < w[1]), "{ev:?}");
}
