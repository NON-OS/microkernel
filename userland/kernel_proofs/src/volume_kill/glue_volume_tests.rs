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

//! The volume as a trip lock, pinned (G8, G9): its declaration and let_go
//! word for word, no spin lock left on it, and keep_switch's take as its
//! one reset.

use super::glue_order_tests::files;
use crate::proofs::proofs_8::transport_takeover::text::{between, code};

const STATE: &str = include_str!("../../../../src/fs/blockfs_volume/state.rs");

#[test]
fn g8_the_volume_is_a_trip_lock_whose_let_go_drops_what_is_held() {
    let decl = concat!(
        "pub(super) static VOLUME: TripLock<Option<VolumeState>> =",
        " TripLock::new(None, let_go);",
    );
    assert!(code(STATE).contains(&code(decl)));
    let rest = concat!(
        "fn let_go(_: &mut Option<VolumeState>, swept: bool) {",
        " crate::fs::cryptoblock::drop_held(swept); }",
        " pub(crate) fn forget_holder(pid: u32) { VOLUME.forget(pid); }",
    );
    assert_eq!(between(STATE, "fn let_go(", "pub(super) static READS:"), code(rest));
}

#[test]
fn g9_no_spin_lock_on_the_volume_and_one_reset() {
    let mut resets = Vec::new();
    for (p, c) in files("fs/blockfs_volume") {
        let name = p.display().to_string();
        // The import stream's LIVE is part D's; until then it keeps its lock.
        if !name.contains("/import_feed/") {
            for lock in ["spin::", "Mutex", "RwLock", "_responsive("] {
                assert!(!c.contains(lock), "{name} holds {lock}");
            }
        }
        for reset in ["volume.take()", "*volume=None", "*v=None", "*guard=None"] {
            if c.contains(reset) {
                resets.push(name.rsplit('/').next().expect("a name").to_string());
            }
        }
    }
    assert_eq!(resets, ["keep_switch.rs"], "keep_switch drops the volume; nothing else does");
    let mut held: Vec<String> = files("fs/cryptoblock")
        .into_iter()
        .filter(|(_, c)| c.contains("Mutex"))
        .map(|(p, _)| p.file_name().expect("a file").to_string_lossy().into_owned())
        .collect();
    held.sort();
    assert_eq!(held, ["pending.rs", "ram.rs"]);
}
