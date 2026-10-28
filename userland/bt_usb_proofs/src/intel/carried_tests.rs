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

use super::loader::carried::carried;
use super::loader::cmds::{boot_params, runs};
use super::loader::ddc::entries;
use super::loader::sfi::layout;

/// Each carried file, the variant it serves, and its Write Boot Params.
const FILES: [(&str, u8, u32, [u8; 3]); 4] = [
    ("ibt-19-0-4", 0x13, 0x0002_4800, [0x53, 0x07, 0x1a]),
    ("ibt-0041-0041", 0x17, 0x0010_0800, [0x18, 0x1d, 0x1a]),
    ("ibt-1040-0041", 0x18, 0x0010_0800, [0x18, 0x1d, 0x1a]),
    ("ibt-0291-0291", 0x1d, 0x0010_0800, [0x91, 0x24, 0x1a]),
];

#[test]
fn every_carried_file_frames_whole_for_every_engine_it_serves() {
    for (stem, variant, addr, build) in FILES {
        let c = carried(stem).unwrap();
        let engines: &[u8] = if variant <= 0x14 { &[0] } else { &[0, 1] };
        for &sbe in engines {
            let cmds = &c.sfi[layout(c.sfi, variant, sbe).unwrap().payload..];
            let all = runs(cmds).unwrap();
            assert_eq!(all.concat(), cmds, "{stem}: the runs are the commands, whole");
            assert!(all.iter().all(|r| r.len() % 4 == 0));
            assert_eq!(boot_params(cmds), Some((addr, build)), "{stem}");
        }
        assert!(entries(c.ddc).is_some_and(|e| !e.is_empty()), "{stem}");
    }
}

#[test]
fn whence_links_answer_with_their_file_and_nothing_else_is_carried() {
    let same = |a: &str, b: &str| std::ptr::eq(carried(a).unwrap().sfi, carried(b).unwrap().sfi);
    let hrp = ["0-0", "0-1", "0-3", "16-4", "32-0", "32-1", "32-4", "240-1", "240-4"];
    assert!(hrp.iter().all(|l| same(&format!("ibt-19-{l}"), "ibt-19-0-4")));
    assert!(same("ibt-1040-4150", "ibt-1040-0041") && same("ibt-1040-1050", "ibt-1040-0041"));
    let others = ["ibt-0040-0041", "ibt-1040-1020", "ibt-20-1-3", "ibt-19-0", "ibt-0041-0041-iml"];
    assert!(others.iter().all(|o| carried(o).is_none()));
}

#[test]
fn commands_are_cut_only_on_aligned_whole_runs() {
    let nop = [0x02, 0xfc, 0x01, 0x00];
    assert_eq!(runs(&nop).unwrap(), [&nop[..]]);
    let two = [0x02, 0xfc, 0x00, 0x02, 0xfc, 0x02, 0, 0];
    assert_eq!(runs(&two).unwrap(), [&two[..]], "3 + 5 octets go together");
    assert!(runs(&nop[..3]).is_none(), "three octets end off the edge");
    assert!(runs(&[0x02, 0xfc, 0x09, 0]).is_none(), "a command past the end");
    assert_eq!(boot_params(&nop), None);
}
