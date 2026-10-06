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

/* /dev's names and what each is. */

use alloc::vec::Vec;

use super::super::synth::Node;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dev {
    Null,
    Zero,
    Full,
    Random,
    Urandom,
    Tty,
}

/* Name, device, major and minor, from Linux's Documentation/admin-guide/devices.txt. */
pub(super) const DEVICES: [(&[u8], Dev, u32, u32); 6] = [
    (b"null", Dev::Null, 1, 3),
    (b"zero", Dev::Zero, 1, 5),
    (b"full", Dev::Full, 1, 7),
    (b"random", Dev::Random, 1, 8),
    (b"urandom", Dev::Urandom, 1, 9),
    (b"tty", Dev::Tty, 5, 0),
];

/* udev's links, which a shell's /dev/stdin redirection opens. */
const LINKS: [(&[u8], &[u8]); 4] = [
    (b"fd", b"/proc/self/fd"),
    (b"stdin", b"/proc/self/fd/0"),
    (b"stdout", b"/proc/self/fd/1"),
    (b"stderr", b"/proc/self/fd/2"),
];

pub fn node(rest: &[&[u8]]) -> Option<Node> {
    match rest {
        [] => {
            let mut names: Vec<Vec<u8>> = DEVICES.iter().map(|d| d.0.to_vec()).collect();
            names.extend(LINKS.iter().map(|l| l.0.to_vec()));
            names.push(b"shm".to_vec());
            Some(Node::Dir(names))
        }
        [name] => DEVICES
            .iter()
            .find(|d| d.0 == *name)
            .map(|d| Node::Dev(d.1))
            .or_else(|| LINKS.iter().find(|l| l.0 == *name).map(|l| Node::Link(l.1.to_vec()))),
        _ => None,
    }
}
