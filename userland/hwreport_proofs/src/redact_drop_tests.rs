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

//! A line that names something personal is dropped whole.

use crate::scrub_run::run;

#[test]
fn a_line_that_names_something_personal_is_dropped_whole() {
    for line in [
        "[WIFI] joined SSID HomeNet",
        "ssid=\"cafe\"",
        "[NVME] serial S4EWNX0R123456",
        "SN: PF2ABCDE",
        "s/n 12345",
        "[GPT] disk UUID follows",
        "Ethernet MAC=00",
        "[USB] passphrase set",
        "Password: x",
        "open /home/ek/notes",
        "hostname box",
        "username root",
        "guid 1",
        "PSK cached",
        "hwaddr is",
        "wwn 5000",
    ] {
        assert_eq!(run(line), None, "{line}");
    }
}
