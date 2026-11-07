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

mod crash_tests;
mod fuzz_tests;
mod live;
mod measure;
mod path_tests;
mod personality_tests;
mod reboot_tests;
mod reference;
mod reference_tests;
mod size_tests;
mod two_client_tests;
mod user_dirs_tests;
mod wire_tests;

use nonos_user_files::limits::DEFAULT_QUOTA;
use nonos_user_files::Broker;

use crate::doubles::{Desk, Volume};

/// The pid every single-client test asks as.
pub const APP: u32 = 7;

pub fn fresh() -> (Broker<Volume>, Desk) {
    (Broker::new(Volume::default(), DEFAULT_QUOTA), Desk::new(&[APP]))
}

/// Commits whatever is owed by moving the clock past its grace.
pub fn settle(b: &mut Broker<Volume>, desk: &Desk) {
    desk.step(10_000_000_000);
    b.tick(desk.now.get()).expect("an owed commit lands");
}

/// A file at `path` holding `body`, synced and closed.
pub fn put_file(b: &mut Broker<Volume>, desk: &Desk, path: &[u8], body: &[u8]) {
    use nonos_user_files::wire::flags::{CREATE, TRUNC, WRITE};
    let mut c = crate::doubles::client(b, desk, APP);
    let (h, _) = c.open(path, WRITE | CREATE | TRUNC, 0o644).expect("opens");
    for (i, piece) in body.chunks(256 << 10).enumerate() {
        c.write(h, (i * (256 << 10)) as u64, piece).expect("writes");
    }
    c.fsync(h).expect("syncs");
    c.close(h).expect("closes");
}

/// The whole of the file at `path`.
pub fn get_file(b: &mut Broker<Volume>, desk: &Desk, path: &[u8]) -> Result<Vec<u8>, i32> {
    use nonos_user_files::wire::flags::READ;
    let mut c = crate::doubles::client(b, desk, APP);
    let (h, a) = c.open(path, READ, 0)?;
    let mut out = vec![0u8; a.size as usize];
    let mut at = 0;
    while at < out.len() {
        let n = c.read(h, at as u64, &mut out[at..])?;
        assert!(n > 0, "a read short of the size");
        at += n;
    }
    c.close(h)?;
    Ok(out)
}

/// `n` bytes no two files share by accident.
pub fn pattern(seed: u8, n: usize) -> Vec<u8> {
    (0..n).map(|i| (i as u8).wrapping_mul(31).wrapping_add(seed) ^ (i >> 13) as u8).collect()
}
