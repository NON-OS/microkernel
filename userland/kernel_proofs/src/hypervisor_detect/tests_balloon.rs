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

//! A balloon the host offers is seen and left off: only virtio-balloon's
//! own ids count, so no other virtio device is taken for one.

use super::balloon::balloon_offered;

#[test]
fn virtio_balloon_is_seen_under_either_id() {
    assert!(balloon_offered([(0x1AF4, 0x1002)].into_iter()));
    assert!(balloon_offered([(0x8086, 0x100E), (0x1AF4, 0x1045)].into_iter()));
}

#[test]
fn no_other_device_is_taken_for_a_balloon() {
    let others = [(0x1AF4, 0x1041), (0x1AF4, 0x1000), (0x1AF4, 0x1052), (0x80EE, 0xCAFE)];
    assert!(!balloon_offered(others.into_iter()));
    assert!(!balloon_offered([(0x1B36, 0x1045)].into_iter()), "the id under another vendor");
    assert!(!balloon_offered(core::iter::empty()));
}
