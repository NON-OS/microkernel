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

//! What the ROM answers after it takes fragments: a Download answer with
//! no index refused, and a patch sent whole that does not run refused.

use super::fixture_setup::{ours, run};
use super::rom_sim::RomSim;
use super::rtl::refused::Refused;
use nonos_bt_hci::HciError;
use nonos_bt_wire::WireError;

#[test]
fn a_download_answered_without_its_index_is_refused() {
    let (table, mut m) = ours();
    let sim = RomSim { terse: true, ..RomSim::rtl8821c() };
    let (why, _) = run(sim, table, &mut m);
    assert_eq!(why, Refused::Hci(HciError::Wire(WireError::Length)));
}

#[test]
fn a_patch_sent_whole_but_not_running_is_refused() {
    let (table, mut m) = ours();
    let sim = RomSim { patched: None, ..RomSim::rtl8821c() };
    let (why, ops) = run(sim, table, &mut m);
    assert_eq!(why, Refused::NotTaken);
    assert_eq!(ops.iter().filter(|&&o| o == 0xfc20).count(), 3);
    assert_eq!(ops.last(), Some(&0x1001));
}
