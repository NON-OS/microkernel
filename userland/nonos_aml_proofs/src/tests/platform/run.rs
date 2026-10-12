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

//! Running the platform layer's evaluations through the real interpreter.

use crate::exec::{ExecLimits, Fault, Machine};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::probe::Probe;

/// The evaluator the capsule would pass, over a probe for the host.
pub fn evaluator(
    probe: &mut Probe,
) -> impl FnMut(&mut Aml, Id, Vec<Object>) -> Result<Object, Fault> + '_ {
    move |aml: &mut Aml, id: Id, args: Vec<Object>| {
        Machine::new(aml, probe, ExecLimits::DEFAULT).evaluate(aml, id, args)
    }
}
