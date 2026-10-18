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

//! Teardown with a card powered, and a setup that fails part way: the card
//! loses its clock and power while the engines still run, the release is
//! still last, and a failed setup holds nothing afterwards.

use nonos_libc::{events, live_grants, refuse_dma_map, set_devices};

use super::rig::{rts5227, sdhc, up};
use crate::card::bring_up;
use crate::error::RtsxError;
use crate::model::with;
use crate::setup::{find, run};

fn last(ev: &[String], what: &str) -> usize {
    ev.iter().rposition(|e| e.starts_with(what)).unwrap_or_else(|| panic!("no {what}: {ev:?}"))
}

#[test]
fn a_powered_card_goes_down_before_the_engines_stop_and_the_release() {
    let mut drv = up(Some(sdhc(1)));
    assert!(bring_up(&mut drv).is_ok() && with(|r| r.powered()));
    drop(drv);
    let ev = events();
    let (power, cmd, dma) =
        (last(&ev, "card power off"), last(&ev, "stop cmd"), last(&ev, "stop dma"));
    let (bier, off, release) =
        (last(&ev, "bier 0x0"), last(&ev, "pci 0x4=0x0402"), last(&ev, "release"));
    assert!(power < cmd && cmd < dma && dma < bier && bier < off && off < release, "{ev:?}");
    assert_eq!(release, ev.len() - 1, "{ev:?}");
    assert_eq!(live_grants(), 0);
}

#[test]
fn a_setup_that_fails_part_way_gives_everything_back() {
    set_devices(vec![rts5227()]);
    refuse_dma_map(1);
    let found = find().expect("listed");
    assert!(matches!(run(&found), Err(RtsxError::DmaMap)));
    let ev = events();
    assert!(
        ev.iter().any(|e| e == "dma_map refused") && ev.iter().all(|e| !e.starts_with("dma_unmap"))
    );
    assert!(ev.iter().all(|e| e != "pci 0x4=0x0406"), "bus master on a refusal: {ev:?}");
    assert_eq!((last(&ev, "release"), live_grants()), (ev.len() - 1, 0), "{ev:?}");
}
