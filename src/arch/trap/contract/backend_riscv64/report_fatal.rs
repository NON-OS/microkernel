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

use crate::arch::trap::contract::cause::TrapCause;
use crate::arch::trap::contract::frame::TrapFrame;
use crate::sys::serial::Line;

use super::{detail, label};

pub(in crate::arch::trap::contract) fn report_fatal<F: TrapFrame>(frame: &F, cause: &TrapCause) {
    let mut l = Line::new();
    l.str(b"!!! KERNEL FATAL TRAP: ").str(label::for_cause(cause).as_bytes()).str(b" !!!");
    l.end_fatal();
    let mut l = Line::new();
    l.str(b"  sepc=").hex(frame.instruction_pointer()).str(b"  sp=").hex(frame.stack_pointer());
    l.str(b"  origin=").str(if frame.from_user() { b"U-mode" } else { b"S-mode" }).end_fatal();
    detail::report(cause);
}
