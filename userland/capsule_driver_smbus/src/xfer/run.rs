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

//! One whole transfer, as Linux's i801_access runs it.

use super::block::block;
use super::check::{check_pre, release, verdict};
use super::kind::Kind;
use super::request::{Xfer, BLOCK_MAX};
use super::wait::wait_done;
use crate::ports::Ports;
use crate::regs::{CNT_START, HSTADD, HSTCMD, HSTCNT, HSTDAT0, HSTDAT1};

/// Runs `x` and returns how many bytes of `out` a read filled. The host is
/// given back to firmware however the transfer ends.
pub fn transfer<P: Ports>(p: &mut P, x: &Xfer, out: &mut [u8; BLOCK_MAX]) -> Result<usize, i32> {
    let r = run(p, x, out);
    let freed = release(p);
    let n = r?;
    freed.map(|_| n)
}

fn run<P: Ports>(p: &mut P, x: &Xfer, out: &mut [u8; BLOCK_MAX]) -> Result<usize, i32> {
    check_pre(p)?;
    p.w8(HSTADD, x.addr << 1 | x.read as u8)?;
    // A quick command has none; a byte send carries its byte here.
    if x.kind != Kind::Quick && !(x.kind == Kind::Byte && x.read) {
        p.w8(HSTCMD, x.cmd)?;
    }
    if x.kind == Kind::Block {
        return block(p, x, out);
    }
    if !x.read {
        match x.kind {
            Kind::ByteData => p.w8(HSTDAT0, x.data[0])?,
            Kind::WordData => {
                p.w8(HSTDAT0, x.data[0])?;
                p.w8(HSTDAT1, x.data[1])?;
            }
            _ => {}
        }
    }
    p.w8(HSTCNT, x.kind.control() | CNT_START)?;
    verdict(wait_done(p)?)?;
    match (x.kind, x.read) {
        (Kind::Byte | Kind::ByteData, true) => {
            out[0] = p.r8(HSTDAT0)?;
            Ok(1)
        }
        (Kind::WordData, true) => {
            out[0] = p.r8(HSTDAT0)?;
            out[1] = p.r8(HSTDAT1)?;
            Ok(2)
        }
        _ => Ok(0),
    }
}
