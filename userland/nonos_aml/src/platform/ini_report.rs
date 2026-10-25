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

//! What the _INI walk did.

use super::eval::{child, Eval};
use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::Id;
use alloc::vec::Vec;

#[derive(Debug, Default)]
pub struct IniReport {
    pub ran: Vec<(Id, Result<(), Fault>)>,
    pub sta_failed: Vec<(Id, Fault)>,
    /// Devices skipped, their subtrees with them.
    pub absent: Vec<Id>,
}

pub(super) fn run_ini(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id, r: &mut IniReport) {
    if let Some(done) = child(aml, eval, dev, *b"_INI", Vec::new()) {
        r.ran.push((dev, done.map(|_| ())));
    }
}
