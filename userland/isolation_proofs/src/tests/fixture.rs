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

//! The sample family, proven once per test run.

use std::sync::OnceLock;

use isolation_stark::{prove, Statement};

pub use crate::sample::witness;
use crate::sample::ENTROPY;

pub fn proven() -> &'static (Statement, Vec<u8>) {
    static ONE: OnceLock<(Statement, Vec<u8>)> = OnceLock::new();
    ONE.get_or_init(|| prove(&witness(), &ENTROPY).expect("a proof"))
}
