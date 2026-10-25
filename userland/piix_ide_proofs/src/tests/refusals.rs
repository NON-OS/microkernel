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

//! A bring-up step the broker refuses gives back everything taken before it.

use nonos_libc::{calls, script_dma, Script};

use super::util::{qemu, up_with};

fn released() -> Vec<(&'static str, u64)> {
    calls().into_iter().filter(|c| c.0.ends_with("release") || c.0 == "dma_unmap").collect()
}

#[test]
fn a_refused_claim_takes_nothing() {
    let (d, _) = up_with(qemu(64), Script { claim: -1, ..Script::default() }, 0);
    assert_eq!(d.err(), Some("claim failed"));
    assert_eq!(calls(), vec![("claim", 3)]);
}

#[test]
fn a_refused_command_write_gives_the_claim_back() {
    let (d, _) = up_with(qemu(64), Script { command: -1, ..Script::default() }, 0);
    assert_eq!(d.err(), Some("pci command setup failed"));
    assert_eq!(released(), vec![("device_release", 3)]);
}

#[test]
fn a_refused_bus_master_window_gives_the_claim_back() {
    let s = Script { ports: [8, 1, 8, 1, 0], ..Script::default() };
    let (d, _) = up_with(qemu(64), s, 0);
    assert_eq!(d.err(), Some("bus master port grant failed"));
    assert_eq!(released(), vec![("device_release", 3)]);
}

#[test]
fn a_short_bus_master_window_is_given_back() {
    let s = Script { ports: [8, 1, 8, 1, 8], ..Script::default() };
    let (d, _) = up_with(qemu(64), s, 0);
    assert_eq!(d.err(), Some("bus master port grant failed"));
    assert_eq!(released(), vec![("pio_release", 54), ("device_release", 3)]);
}

#[test]
fn a_refused_secondary_control_window_gives_back_every_grant() {
    let s = Script { ports: [8, 1, 8, 0, 16], ..Script::default() };
    let (d, _) = up_with(qemu(64), s, 0);
    assert_eq!(d.err(), Some("control block port grant failed"));
    let mut r = released();
    let last = r.pop().unwrap();
    assert_eq!(last, ("device_release", 3), "the claim goes last");
    r.sort();
    assert_eq!(
        r,
        vec![("pio_release", 50), ("pio_release", 51), ("pio_release", 52), ("pio_release", 54)]
    );
}

#[test]
fn a_refused_dma_grant_gives_back_every_window_and_the_claim() {
    script_dma(Err(-12));
    let (d, _) = up_with(qemu(64), Script::default(), 0);
    assert_eq!(d.err(), Some("dma map failed"));
    let r = released();
    assert_eq!(r.len(), 6, "{r:?}");
    assert_eq!(r.last(), Some(&("device_release", 3)));
}

#[test]
fn a_dma_grant_above_4_gib_is_unmapped() {
    script_dma(Ok((0xFFFF_8000, 0)));
    let (d, _) = up_with(qemu(64), Script::default(), 0);
    assert_eq!(d.err(), Some("dma map failed"));
    assert!(released().contains(&("dma_unmap", 70)));
}

#[test]
fn a_short_dma_grant_is_unmapped() {
    script_dma(Ok((0x0010_0000, 0x1000)));
    let (d, _) = up_with(qemu(64), Script::default(), 0);
    assert_eq!(d.err(), Some("dma map failed"));
    assert!(released().contains(&("dma_unmap", 70)));
}

#[test]
fn release_gives_back_the_dma_grant_every_window_and_the_claim() {
    let (d, _) = up_with(qemu(64), Script::default(), 0);
    d.expect("bring-up").release();
    let r = released();
    assert_eq!(r.first(), Some(&("dma_unmap", 70)));
    assert_eq!(r.last(), Some(&("device_release", 3)));
    assert_eq!(r.len(), 7);
}
