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

//! Calls 200 deep, as firmware that chains helpers reaches, run on a 2 MiB
//! stack like the capsule's: they complete, called plainly or from inside
//! nested expressions; 257 deep, unbounded recursion, and a chain whose
//! nesting would outrun the stack share all stop with CallDepth, never a
//! stack overflow.

use super::deep_rig::on_capsule_stack;
use crate::enc::*;
use crate::exec::{ExecLimits, FaultKind};
use crate::probe::Probe;
use crate::run::{boot, call_with};

#[test]
fn two_hundred_deep_completes_plain_and_nested() {
    assert_eq!(on_capsule_stack(200, false, ExecLimits::DEFAULT), Ok(7));
    assert_eq!(on_capsule_stack(200, true, ExecLimits::DEFAULT), Ok(7));
}

#[test]
fn past_the_depth_or_the_stack_share_stops_cleanly() {
    assert_eq!(on_capsule_stack(300, false, ExecLimits::DEFAULT), Err(FaultKind::CallDepth));
    let small = ExecLimits { stack_bytes: 64 << 10, ..ExecLimits::DEFAULT };
    assert_eq!(on_capsule_stack(100, true, small), Err(FaultKind::CallDepth), "the stack share");
    let forever = method("\\LOOP", 0, &ret(&name("\\LOOP")));
    let r = std::thread::Builder::new().stack_size(2 << 20).spawn(move || {
        let (mut aml, _) = boot(&forever, 2);
        call_with(&mut aml, &mut Probe::default(), ExecLimits::DEFAULT, "\\LOOP", vec![])
            .map(|_| ())
            .map_err(|f| f.kind)
    });
    assert_eq!(r.unwrap().join().unwrap().err(), Some(FaultKind::CallDepth));
}
