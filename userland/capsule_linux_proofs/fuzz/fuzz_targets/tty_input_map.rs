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

//! Any bytes as a terminal's settings and what it sends: each byte read is
//! the one typed, mapped as c_iflag says, or none for a carriage return
//! under IGNCR (nonos_capsule_linux_proofs::console::input_map_check).

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_capsule_linux_proofs::console::input_map_check::check_bytes;

fuzz_target!(|data: &[u8]| {
    check_bytes(data);
});
