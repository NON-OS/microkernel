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

//! Loading the definition blocks (ACPI 6.5, 5.2.11.2): the DSDT, then each
//! SSDT. A refused SSDT adds nothing and the rest load; a refused DSDT
//! leaves no AML served at all (`load_rule`).

use super::load_rule::{after_refusal, refused_line, Next};
use super::tables::Tables;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use crate::sys::log_parts::emit;
use nonos_aml::arena::Limits;
use nonos_aml::error::ErrorKind;
use nonos_aml::exec::{ExecLimits, Machine};
use nonos_aml::load::Aml;

pub fn load(t: &Tables, host: &mut CapsuleHost) -> Aml {
    let mut aml = Aml::new(Limits::DEFAULT);
    for (i, block) in t.blocks.iter().enumerate() {
        let r =
            Machine::new(&mut aml, host, ExecLimits::DEFAULT).load_table(&mut aml, block.clone());
        let Err(e) = r else { continue };
        let cut = e.kind == ErrorKind::Budget;
        emit(|out| refused_line(i, cut, e.at, out));
        if after_refusal(i) == Next::NoAml {
            return Aml::new(Limits::DEFAULT);
        }
    }
    Line::new(b"namespace: ").dec(aml.ns.len() as u64).str(b" objects").end();
    aml
}
