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
use alloc::vec::Vec;
use nonos_aml::exec::{ExecLimits, Fault, Machine};
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::object::Object;

pub fn evaluate(
    aml: &mut Aml,
    host: &mut CapsuleHost,
    id: Id,
    args: Vec<Object>,
) -> Result<Object, Fault> {
    Machine::new(aml, host, ExecLimits::DEFAULT).evaluate(aml, id, args)
}

/// The evaluator the platform layer takes.
pub fn evaluator(
    host: &mut CapsuleHost,
) -> impl FnMut(&mut Aml, Id, Vec<Object>) -> Result<Object, Fault> + '_ {
    move |aml, id, args| evaluate(aml, host, id, args)
}
