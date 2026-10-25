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

//! The device half of a market proof: only a pinned registrar's root, at its
//! depth and inside its life, is accepted.

use nonos_proof_abi::trust::DeviceRoot;

use crate::assemble::enrolled::enrolled_where;
use crate::assemble::error::Refusal;
use crate::fixture::device;
use crate::market::device::{registrar, root_bytes};

const DAY: u64 = 20_372;

fn pinned() -> DeviceRoot {
    let r = device().registry;
    DeviceRoot {
        root: r.root().map(|f| f.to_u64()),
        depth: r.depth(),
        from_day: DAY - 5,
        life_days: 30,
    }
}

#[test]
fn only_a_pinned_registrar_inside_its_life_is_accepted() {
    let (reg, pin) = (device().registry, pinned());
    let (root, depth) = (reg.root(), reg.depth());
    assert_eq!(registrar(&[pin], DAY, root, depth), Some(pin));
    assert_eq!(root_bytes(&pin), crate::fixture::root_bytes(&reg));
    for day in [DAY - 6, DAY + 26] {
        assert_eq!(registrar(&[pin], day, root, depth), None, "day {day}");
    }
    assert_eq!(registrar(&[pin], DAY, root, depth + 1), None, "another depth");
    let other = DeviceRoot { root: [1, 2, 3, 4], ..pin };
    assert_eq!(registrar(&[other], DAY, root, depth), None, "another root");
    assert_eq!(registrar(&[other, pin], DAY, root, depth), Some(pin));
}

#[test]
fn a_transcript_whose_root_is_not_pinned_is_refused() {
    let d = device();
    let eks: Vec<&[u8]> = d.eks.iter().map(Vec::as_slice).collect();
    let pin = pinned();
    let ok = |roots: &[DeviceRoot]| {
        enrolled_where(&d.transcript, |r, n| registrar(roots, DAY, r, n).is_some(), &eks)
    };
    assert_eq!(ok(&[pin]).map(|e| e.depth).ok(), Some(pin.depth));
    let stale = DeviceRoot { from_day: DAY - 40, ..pin };
    assert_eq!(ok(&[stale]).err(), Some(Refusal::RootNotRequested));
    assert_eq!(ok(&[]).err(), Some(Refusal::RootNotRequested));
}
