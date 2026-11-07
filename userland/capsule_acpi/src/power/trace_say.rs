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

//! The bring-up trace's power lines: each battery's _STA as it answered (or
//! the fault), whether _BIX/_BIF and _BST were read, the charge the kernel
//! is told; each adapter's _PSR.

use super::state::{Battery, Power};
use super::word::word;
use crate::boot::path;
use nonos_aml::exec::Fault;
use nonos_aml::load::Aml;

pub fn battery(aml: &Aml, b: &Battery, sta: &Result<u64, Fault>) {
    let Some(l) = crate::trace::line() else { return };
    let l = l.str(b"battery ").str(path(aml, b.id).as_bytes());
    let l = match sta {
        Ok(s) => l.str(b" sta=").hex(*s),
        Err(f) => l
            .str(b" sta=fault ")
            .str(alloc::format!("{:?}", f.kind).as_bytes())
            .str(b" at ")
            .hex(f.at as u64),
    };
    let ok = |b: bool| -> &[u8] {
        if b {
            b"read"
        } else {
            b"none"
        }
    };
    l.str(b" present=")
        .dec(u64::from(b.present))
        .str(b" info=")
        .str(ok(b.info.is_some()))
        .str(b" status=")
        .str(ok(b.status.is_some()))
        .end();
}

pub fn adapters(aml: &Aml, p: &Power) {
    for (id, on) in &p.adapters {
        let Some(l) = crate::trace::line() else { return };
        let l = l.str(b"ac ").str(path(aml, *id).as_bytes()).str(b" psr=");
        l.str(match on {
            Some(true) => b"1",
            Some(false) => b"0",
            None => b"none",
        })
        .end();
    }
    if let Some(l) = crate::trace::line() {
        l.str(b"power word=").hex(word(p)).end();
    }
}
