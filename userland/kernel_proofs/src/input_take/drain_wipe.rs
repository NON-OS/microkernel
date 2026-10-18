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

//! MkInputEventDrain leaves no keystroke on its kernel stack: the scratch
//! events are wiped, volatile, after the copy out and before either return.

const DRAIN: &str = include_str!("../../../../src/syscall/dispatch/router/input_ops/do_drain.rs");

#[test]
fn the_drain_wipes_its_scratch_after_the_copy_on_both_paths() {
    let copy = DRAIN.find("copy_to_user(").expect("the copy out");
    let wipe = DRAIN.find("write_volatile(e, InputEvent::default())").expect("the wipe");
    let fence = DRAIN.find("compiler_fence(").expect("the fence");
    let fault = DRAIN.find("if copied.is_err()").expect("the fault return");
    assert!(copy < wipe && wipe < fence && fence < fault, "copy, wipe, fence, then EFAULT");
    assert!(DRAIN.contains("for e in scratch[..n].iter_mut()"), "every event drained");
}
