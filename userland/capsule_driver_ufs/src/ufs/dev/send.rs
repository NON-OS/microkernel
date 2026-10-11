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

//! A device management request: no data, logical unit 0.

use crate::ufs::ctx::Ctx;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::utp::issue::{issue, Req};
use crate::ufs::utp::layout::RSP_READ;
use crate::ufs::utp::utrd::Dir;

pub fn send<H: Hw>(hw: &H, c: &Ctx, upiu: &[u8], ms: u64) -> Result<[u8; RSP_READ], UfsError> {
    let req = Req { cmd_type: c.dev_type(), dir: Dir::None, upiu, data_len: 0, ms };
    issue(hw, c, &req)
}
