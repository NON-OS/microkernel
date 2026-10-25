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

//! Expressions, section 20.2.5.4.

use super::{Arg::*, Class, Group};

const fn g(args: &'static [super::Arg], ops: &'static [u16]) -> Group {
    Group { class: Class::Expr, pkg: false, args, ops }
}

pub const EXPRS: &[Group] = &[
    // Operand, Operand, Target: the arithmetic and bitwise operators,
    // Concatenate, ConcatenateResTemplate and Index.
    g(
        &[Term, Term, Super],
        &[0x72, 0x73, 0x74, 0x77, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F, 0x84, 0x85, 0x88],
    ),
    // Divide: a Remainder target, then a Quotient target.
    g(&[Term, Term, Super, Super], &[0x78]),
    // Operand, Target: Not, FindSetLeftBit, FindSetRightBit, the ToX
    // conversions, ToBCD and FromBCD.
    g(&[Term, Super], &[0x80, 0x81, 0x82, 0x96, 0x97, 0x98, 0x99, 0x5B28, 0x5B29]),
    g(&[Term, Term, Super], &[0x9C]),
    g(&[Term, Term, Term, Super], &[0x9E]),
    // LAnd, LOr, LEqual, LGreater, LLess.
    g(&[Term, Term], &[0x90, 0x91, 0x93, 0x94, 0x95]),
    // LNot, DerefOf.
    g(&[Term], &[0x92, 0x83]),
    g(&[Term, Super], &[0x70]),
    g(&[Term, Simple], &[0x9D]),
    // Increment, Decrement, RefOf, SizeOf, ObjectType.
    g(&[Super], &[0x75, 0x76, 0x71, 0x87, 0x8E]),
    g(&[Super, Super], &[0x5B12]),
    // Acquire: the mutex, then a WordData timeout in milliseconds.
    g(&[Super, U16], &[0x5B23]),
    g(&[Super, Term], &[0x5B25]),
    // Match: the package, MatchOpcode, Operand, MatchOpcode, Operand, the
    // start index.
    g(&[Term, U8, Term, U8, Term, Term], &[0x89]),
    g(&[Name, Super], &[0x5B20]),
    g(&[Term, Term, Term, Term, Term, Term], &[0x5B1F]),
    g(&[], &[0x5B33]),
];
