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

//! Setup's answers as setup keeps them, told apart by their time zone, and
//! a running boot's vfs holding them. Mounted beside `fake_vfs`.

use nonos_policy_proto::setup_record::{Answers, Name, Tier};

use crate::fake_vfs::FakeVfs;

pub const ANSWERS: &str = "/nonos/setup/answers";

pub fn answers(timezone: i8) -> Vec<u8> {
    let (username, qwen_tier) = (Name::new(b"ada").unwrap(), Tier::new(b"small").unwrap());
    let a = Answers { keyboard_layout: 2, timezone, wallpaper: 3, username, qwen_tier };
    a.encode().to_vec()
}

pub fn vfs_with(raw: &[u8]) -> FakeVfs {
    let mut v = FakeVfs::default();
    v.put(ANSWERS, raw);
    v
}
