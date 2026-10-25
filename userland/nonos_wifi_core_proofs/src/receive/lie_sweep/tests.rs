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

//! A chip's lies about whole protected frames, each said decrypted: a
//! replayed or older packet number, a key index no installed key has, the
//! group key's own counter, and, where the station checks the MIC itself,
//! a MIC that fails or a group frame under the pairwise key.

use super::run::sweep;
use crate::receive::ccmp_tests::tk;
use nonos_wifi_core::station::Ccmp;

#[test]
fn a_station_that_decrypts_receive_itself_is_not_moved_by_a_chips_lie() {
    sweep(|| Ccmp::Software { tk: tk() }, true);
}

#[test]
fn a_station_that_left_receive_to_the_chip_still_holds_counters_and_key_indexes() {
    sweep(|| Ccmp::SoftwareTxHwRx { tk: tk() }, false);
}
