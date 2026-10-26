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

//! What the pseudo-terminal tests share: a pair as foot leaves it once its
//! shell runs (unlocked, a slave open), and the settings changed by flag.

use super::flags::Termios;
use super::pair::Pair;
use super::table::Table;

pub const EAGAIN: i64 = 11;
pub const EIO: i64 = 5;
pub const ENOENT: i64 = 2;

/// A table holding pair 0, unlocked with its slave open, foreground `fg`.
pub fn table(fg: u32) -> Table {
    let mut t = Table::new();
    assert_eq!(t.make(), Some(0));
    let p = t.get(0).unwrap();
    (p.locked, p.slave, p.slave_seen, p.fg) = (false, true, true, fg);
    t
}

pub fn pair() -> Pair {
    let mut t = table(0);
    t.pairs[0].take().unwrap()
}

/// The settings with the c_lflag (`at` 12) or c_oflag (`at` 4) bits `off`
/// cleared.
pub fn without(t: Termios, at: usize, off: u32) -> Termios {
    let mut t = t;
    let w = u32::from_le_bytes(t[at..at + 4].try_into().unwrap()) & !off;
    t[at..at + 4].copy_from_slice(&w.to_le_bytes());
    t
}

/// Bytes typed at the master, and what the master then reads: the echo.
pub fn type_in(p: &mut Pair, bytes: &[u8]) -> Vec<u8> {
    let (n, _) = p.master_write(bytes).unwrap();
    assert_eq!(n, bytes.len());
    p.master_read(1 << 20).unwrap_or_default()
}

/// One read on the slave, as many bytes as it may give.
pub fn read(p: &mut Pair) -> Result<Vec<u8>, i64> {
    p.slave_read(1 << 20)
}
