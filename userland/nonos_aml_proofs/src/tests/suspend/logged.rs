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

//! The evaluator the capsule would pass, logging each evaluation it is
//! asked for on the model's timeline.

use super::model::Log;
use crate::exec::{ExecLimits, Fault, Machine};
use crate::load::Aml;
use crate::namespace::{write_path, Id};
use crate::object::Object;
use crate::probe::Probe;

/// The evaluator, logging each evaluation it is asked for on the timeline.
pub fn logged<'a>(
    probe: &'a mut Probe,
    log: Log,
) -> impl FnMut(&mut Aml, Id, Vec<Object>) -> Result<Object, Fault> + 'a {
    move |aml: &mut Aml, id: Id, args: Vec<Object>| {
        let mut path = String::new();
        let _ = write_path(&aml.ns, id, &mut path);
        // Integers as numbers, a buffer by its first byte (a UUID's tells
        // the _DSM sets apart).
        let ints: Vec<String> = args
            .iter()
            .filter_map(|a| match a {
                Object::Integer(n) => Some(n.to_string()),
                Object::Buffer(b) => b.borrow().first().map(|x| format!("b{x:02x}")),
                _ => None,
            })
            .collect();
        log.borrow_mut().push(format!("{path}({})", ints.join(",")));
        Machine::new(aml, probe, ExecLimits::DEFAULT).evaluate(aml, id, args)
    }
}
