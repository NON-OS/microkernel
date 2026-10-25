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

//! The scrub on lines a boot log really has, and on every shape it masks.

use crate::scrub_run::run;

#[test]
fn every_shape_of_value_is_masked_wherever_it_sits() {
    let masked = [
        ("[NET] station 52:54:00:12:34:56 up", "[NET] station <x> up"),
        ("addr 52-54-00-12-34-56.", "addr <x>."),
        ("cisco 5254.0012.3456", "cisco <x>"),
        ("dhcp 10.0.2.15/24 via 10.0.2.2.", "dhcp <x>/24 via <x>."),
        ("ip:192.168.1.10,", "ip:<x>,"),
        ("glued ip10.0.2.15 here", "glued ip<x> here"),
        ("v6 fe80::5054:ff:fe12:3456 up", "v6 <x> up"),
        ("full 2001:0db8:0000:0000:0000:0000:0000:0001", "full <x>"),
        ("disk 123e4567-e89b-12d3-a456-426614174000", "disk <x>"),
        ("key 0xdeadbeefcafebabe1234", "key 0x<x>"),
        ("va ffff800000123456 mapped", "va <x> mapped"),
        ("bare 525400123456", "bare <x>"),
        ("id 356938035643809", "id <x>"),
        ("then hex aa:bb:cc:dd:ee:ff0", "then hex <x>0"),
    ];
    for (line, want) in masked {
        assert_eq!(run(line).as_deref(), Some(want), "{line}");
    }
}

#[test]
fn the_lines_a_report_is_for_are_kept_as_they_are() {
    for line in [
        "[DRIVER-HDA] capsule spawned",
        "[IOMMU] none present, enforcing=0, unconfined grants=3, posture=dma-unconfined",
        "[PCI] 00:1f.3 8086:a348 class 04.03",
        "[VT-D] refused: no remapping unit confines it",
        "[SMP] machine has 8 cpus, version 0.9.3",
        "[EXIT] driver.e1000e0 status 2: no device present, not started",
        "[RTL8821CE] firmware 0x0a01 loaded",
    ] {
        assert_eq!(run(line).as_deref(), Some(line), "{line}");
    }
}

#[test]
fn bytes_outside_printable_ascii_never_reach_the_report() {
    assert_eq!(run("tab\there\u{7f}é\x1b[31m").unwrap(), "tab?here????[31m");
}
