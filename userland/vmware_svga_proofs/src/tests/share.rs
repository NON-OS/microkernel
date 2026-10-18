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

//! the share names the owner, and a share the kernel refuses names no
//! owner at all.

use super::util::up;
use crate::bench::Bench;
use crate::protocol::Request;
use crate::server::dispatch::dispatch;
use nonos_libc::{fail_share, shares};

#[test]
fn a_refused_share_names_no_owner() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |_| {});
    let mut tx = [0u8; 64];
    fail_share();
    let get_primary = Request { op: 0x0C, flags: 0, request_id: 9 };
    assert_eq!(dispatch(&mut d, 40, &get_primary, &[], &mut tx), Some(-110));
    assert_eq!((d.owner, d.handle), (0, 0), "nobody may draw");
    assert!(shares().is_empty());
}
