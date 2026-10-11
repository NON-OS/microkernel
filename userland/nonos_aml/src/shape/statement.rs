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

//! Statements, section 20.2.5.3.

use super::{Arg::*, Class, Group};

const fn g(pkg: bool, args: &'static [super::Arg], ops: &'static [u16]) -> Group {
    Group { class: Class::Statement, pkg, args, ops }
}

pub const STATEMENTS: &[Group] = &[
    // DefIfElse: the predicate and its TermList; an Else after the package
    // belongs to it.
    g(true, &[Term, Terms], &[0xA0]),
    g(true, &[Term, Terms], &[0xA2]),
    Group { class: Class::Else, pkg: true, args: &[Terms], ops: &[0xA1] },
    // Break, Continue, Noop, BreakPoint.
    g(false, &[], &[0xA5, 0x9F, 0xA3, 0xCC]),
    g(false, &[Super, Term], &[0x86]),
    // Release, Reset, Signal, Unload.
    g(false, &[Super], &[0x5B27, 0x5B26, 0x5B24, 0x5B2A]),
    // Return, Sleep, Stall.
    g(false, &[Term], &[0xA4, 0x5B22, 0x5B21]),
    // DefFatal: FatalType, FatalCode, FatalArg.
    g(false, &[U8, U32, Term], &[0x5B32]),
];
