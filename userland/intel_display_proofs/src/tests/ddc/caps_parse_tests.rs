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

//! Reading a capabilities string: the top-level vcp list alone, its codes
//! and the values after the input source and the power mode, any spacing
//! and either case; a string with a malformed list, or none, gives nothing.

use crate::ddc::caps::Caps;

fn codes(c: &Caps) -> Vec<u8> {
    (0..=255u8).filter(|x| c.has(*x)).collect()
}

#[test]
fn the_vcp_list_gives_its_codes_and_listed_values() {
    let s = b"(prot(monitor)type(lcd)cmds(01 02 03 F3)vcp(02 10 12 14(05 08 0B) 60(0F 11 12) d6(01 04 05))mswhql(1))";
    let c = Caps::parse(s).unwrap();
    assert_eq!(codes(&c), [0x02, 0x10, 0x12, 0x14, 0x60, 0xD6], "values are not codes");
    assert!(c.lists(0x60, 0x11) && !c.lists(0x60, 0x10) && !c.lists(0x60, 0x0111));
    assert!(c.lists(0xD6, 5) && !c.lists(0x14, 5), "values kept for two codes alone");
    assert_eq!(
        (c.listed(0x60), c.listed(0xD6), c.listed(0x14)),
        (&[0x0F, 0x11, 0x12][..], &[1, 4, 5][..], &[][..])
    );
    let tight = Caps::parse(b"(VCP(021012)mccs_ver(2.1))").unwrap();
    assert_eq!(codes(&tight), [0x02, 0x10, 0x12], "no spacing, upper case key");
}

#[test]
fn only_the_top_level_vcp_key_counts() {
    assert_eq!(Caps::parse(b"(vcpname(10(Brightness))cmds(01))"), None);
    assert_eq!(Caps::parse(b"(model(x vcp(10)))"), None, "nested in another key");
    assert_eq!(Caps::parse(b"(prot(monitor))"), None, "no list at all");
}

#[test]
fn a_malformed_list_gives_nothing() {
    for s in
        [&b"(vcp(10 1))"[..], b"(vcp(10 G2))", b"(vcp(10(01(02))))", b"(vcp(10 12", b"(vcp((01)))"]
    {
        assert_eq!(Caps::parse(s), None, "{}", String::from_utf8_lossy(s));
    }
}

#[test]
fn values_past_sixteen_are_dropped_not_overrun() {
    let many: String = (0..40).map(|v| format!("{v:02X} ")).collect();
    let c = Caps::parse(format!("(vcp(60({many})))").as_bytes()).unwrap();
    assert!(c.lists(0x60, 0x0F) && !c.lists(0x60, 0x10));
    assert_eq!(c.listed(0x60), (0..16).collect::<Vec<u8>>());
}
