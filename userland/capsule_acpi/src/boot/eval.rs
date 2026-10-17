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

//! One evaluation, with a fresh budget, under the capsule's host.

use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};
use nonos_aml::exec::{ExecLimits, Fault, Machine};
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::object::Object;

/// Evaluations whose leftover mutexes, or whose loads' _REG faults, are
/// said, each kind; past it they are counted only. Our bound, from no
/// source: a method run at every SCI must not fill the log.
const TELL: u32 = 16;
static TOLD: AtomicU32 = AtomicU32::new(0);
static TOLD_REG: AtomicU32 = AtomicU32::new(0);

pub fn evaluate(
    aml: &mut Aml,
    host: &mut CapsuleHost,
    id: Id,
    args: Vec<Object>,
) -> Result<Object, Fault> {
    let (before, tables, reg) = (aml.rt.left_held, aml.tables.len(), aml.rt.reg_faults);
    let r = Machine::new(aml, host, ExecLimits::DEFAULT).evaluate(aml, id, args);
    // A table loaded at run time may define or redefine \_S5 (F-111).
    if aml.tables.len() > tables {
        crate::sleep::read_s5(aml, host);
    }
    let left = aml.rt.left_held.wrapping_sub(before);
    if left > 0 && TOLD.fetch_add(1, Ordering::Relaxed) < TELL {
        let line = Line::new(b"AML: method ").hex(u64::from(id)).str(b" ended holding ");
        line.dec(u64::from(left))
            .str(b" mutexes; released, in all ")
            .dec(u64::from(aml.rt.left_held))
            .end();
    }
    let failed = aml.rt.reg_faults.wrapping_sub(reg);
    if failed > 0 && TOLD_REG.fetch_add(1, Ordering::Relaxed) < TELL {
        let line = Line::new(b"AML: method ").hex(u64::from(id)).str(b" loaded a table; _REG ");
        line.str(b"faulted in ")
            .dec(u64::from(failed))
            .str(b" scopes, the load kept; in all ")
            .dec(u64::from(aml.rt.reg_faults))
            .end();
    }
    r
}

/// The evaluator the platform layer takes.
pub fn evaluator(
    host: &mut CapsuleHost,
) -> impl FnMut(&mut Aml, Id, Vec<Object>) -> Result<Object, Fault> + '_ {
    move |aml, id, args| evaluate(aml, host, id, args)
}
