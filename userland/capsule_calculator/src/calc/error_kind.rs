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

//! Why a calculation stopped. Kept apart from the rest of the state so the
//! arithmetic that raises it and the words that show it can be proven on the
//! host without the window behind them.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ErrorKind {
    None,
    DivByZero,
    DomainError,
    Overflow,
}
