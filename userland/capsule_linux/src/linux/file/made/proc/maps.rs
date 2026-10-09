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

/*
 * /proc/<pid>/maps: one line for each region the personality keeps for the
 * process, in address order, in Linux's layout.
 *
 * The region list is the truth about the address space (guest/region.rs):
 * nothing is added to it here and nothing merged. A PROT_NONE reservation
 * shows as ---p. The stack and the heap are named, as Linux names them;
 * the rest is shown as anonymous, which is all the list records of it.
 */

use alloc::string::String;
use alloc::vec::Vec;

use crate::linux::guest::STACK_TOP;

use super::super::view::Proc;

/*
 * Where Linux starts the name: 25 columns plus six for each of the two
 * addresses on a 64-bit machine, less one.
 */
const NAME_AT: usize = 25 + 6 * 8 - 1;

pub fn maps(p: &Proc) -> Vec<u8> {
    let mut regions = p.regions.clone();
    regions.sort_by_key(|r| r.at);
    let mut out = String::new();
    for r in regions {
        let bit = |on: bool, c: char| if on && r.backed { c } else { '-' };
        let perms = alloc::format!("{}{}{}p", bit(true, 'r'), bit(r.write, 'w'), bit(r.exec, 'x'));
        let mut line = alloc::format!("{:08x}-{:08x} {perms} 00000000 00:00 0", r.at, r.at + r.len);
        let end = r.at + r.len;
        let name = match () {
            _ if end == STACK_TOP => Some("[stack]"),
            _ if r.at >= p.brk.0 && end <= p.brk.1.max(p.brk.0) && r.len > 0 => Some("[heap]"),
            _ => None,
        };
        if let Some(name) = name {
            while line.len() < NAME_AT {
                line.push(' ');
            }
            line.push(' ');
            line.push_str(name);
        }
        out.push_str(&line);
        out.push('\n');
    }
    out.into_bytes()
}
