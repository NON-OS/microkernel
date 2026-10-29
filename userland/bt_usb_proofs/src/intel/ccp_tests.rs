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

//! The AX200 (8087:0029): CcP, variant 0x14, in the old version form,
//! taking ibt-20-<hw revision>-<fw revision>.

use super::loader::carried::carried;
use super::loader::ddc::entries;
use super::model::Intel;
use super::rig::run;
use super::rig_ops::ops_after;

fn ax200(hw: u8, fw: u8, file: &str) -> Intel {
    let old = |fv: u8| vec![0x37, 0x14, hw, fv, fw, 0xc1, 0x21, 0x18, 0];
    let both = [old(0x06), old(0x23)];
    let expect = carried(file).unwrap().sfi.len() - 644;
    let p = vec![0u8; 22];
    Intel { tlv: both.clone(), old: both, boot_params: p, boots: true, expect, ..Intel::default() }
}

#[test]
fn an_ax200_takes_the_ibt_20_file_its_revisions_name() {
    for (hw, fw, file) in [(1, 3, "ibt-20-1-3"), (0, 3, "ibt-20-1-3"), (1, 4, "ibt-20-1-4")] {
        let (r, m, said) = run(ax200(hw, fw, file), 0x0029);
        assert_eq!(r, Ok(()), "{file}");
        let c = carried(file).unwrap();
        let sent: Vec<u8> = m.fragments.iter().flat_map(|f| f[1..].to_vec()).collect();
        assert_eq!(sent, [&c.sfi[..384], &c.sfi[388..]].concat(), "RSA header, then 644 on");
        assert_eq!(m.boot_addrs, [0x0002_4800]);
        assert_eq!(m.ddc, entries(c.ddc).unwrap());
        assert_eq!(ops_after(&m, 0xfc01), [0xfc8b, 0xfc8b, 0xfc52, 0x0c03]);
        assert_eq!(said[0], format!("intel: sending ibt-20-{hw}-{fw}"));
    }
}

#[test]
fn an_ax200_running_the_carried_build_is_left_running() {
    let mut m = ax200(1, 4, "ibt-20-1-4");
    m.booted = true;
    let (r, m, _) = run(m, 0x0029);
    assert_eq!((r, m.ops), (Ok(()), vec![0xfc05, 0xfc52, 0x0c03]));
}
