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

use alloc::string::String;
use alloc::vec::Vec;

use super::complex::Selector;
use super::state::{FormState, UserState};

/* A pseudo-class on a compound, evaluated against the document. */
#[derive(Clone)]
pub enum Pseudo {
    FirstChild,
    LastChild,
    OnlyChild,
    FirstOfType,
    LastOfType,
    OnlyOfType,
    /* :nth-child(An+B), 1-based among element siblings. */
    NthChild(i32, i32),
    NthLastChild(i32, i32),
    NthOfType(i32, i32),
    NthLastOfType(i32, i32),
    /* :nth-child(An+B of S): counted among the siblings matching S. */
    NthChildOf(i32, i32, Vec<Selector>),
    NthLastChildOf(i32, i32, Vec<Selector>),
    Empty,
    Root,
    Scope,
    /* :is(), :where() and :-webkit-any(): some argument matches. */
    Matches(Vec<Selector>),
    /* :not(): no argument matches. */
    Not(Vec<Selector>),
    /* :has(): an element related to this one as a relative argument says
     * matches it. Each argument ends in a step whose compound is HasAnchor. */
    Has(Vec<Selector>),
    /* The element a :has() argument is anchored at; never written. */
    HasAnchor,
    AnyLink,
    Lang(String),
    /* :dir(rtl) when true, :dir(ltr) when false. */
    Dir(bool),
    Defined,
    Open,
    Form(FormState),
    User(UserState),
    /* A condition that cannot hold in a NONOS document: :visited, shadow
     * tree, media playback, fullscreen and autofill states among others. */
    Never,
}
