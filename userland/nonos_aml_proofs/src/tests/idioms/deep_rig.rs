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

//! A chain of methods each calling the next, run on a 2 MiB stack like
//! the capsule's.

use super::more::*;
use crate::enc::*;
use crate::exec::{ExecLimits, FaultKind};
use crate::probe::Probe;
use crate::run::{boot, call_with};

/// C000 calls C001 ... up to C(n-1), which returns 7; `nested` puts each
/// call four terms down: If (LEqual (Add (Multiply (Cnnn (), One), Zero), 7)).
pub fn chain(n: usize, nested: bool) -> Vec<u8> {
    let mut t = Vec::new();
    for i in 0..n {
        let next = name(&format!("\\C{:03}", i + 1));
        let body = match (i + 1 == n, nested) {
            (true, _) => ret(&int(7)),
            (false, false) => ret(&next),
            (false, true) => {
                let deep =
                    e(LEQUAL, &[&bin(ADD, &e(0x77, &[&next, &[0x01], &[0x00]]), &[0x00]), &int(7)]);
                cat(&[&if_(&deep, &ret(&int(7))), &ret(&int(0))])
            }
        };
        t.extend(method(&format!("\\C{:03}", i), 0, &body));
    }
    t
}

pub fn on_capsule_stack(n: usize, nested: bool, limits: ExecLimits) -> Result<u64, FaultKind> {
    std::thread::Builder::new()
        .stack_size(2 << 20)
        .spawn(move || {
            let (mut aml, _) = boot(&chain(n, nested), 2);
            let r = call_with(&mut aml, &mut Probe::default(), limits, "\\C000", vec![]);
            r.map(|v| crate::run::num(&v)).map_err(|f| f.kind)
        })
        .unwrap()
        .join()
        .unwrap()
}
