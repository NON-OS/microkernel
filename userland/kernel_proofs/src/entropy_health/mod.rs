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

//! The entropy capsule's bytes on their way into a seed: the kernel's
//! own health tests, accumulator, credit rule and ask rule, by path.

#[path = "../../../../src/crypto/util/rng/entropy/accumulator.rs"]
pub mod accumulator;
#[cfg(test)]
mod accumulator_tests;
#[path = "../../../../src/security/entropy_capsule/client/ask_rule.rs"]
pub mod ask_rule;
#[cfg(test)]
mod ask_tests;
#[cfg(test)]
pub(crate) mod bytes;
#[path = "../../../../src/crypto/util/rng/entropy/credit.rs"]
pub mod credit;
#[path = "../../../../src/crypto/util/rng/entropy/health.rs"]
pub mod health;
#[cfg(test)]
mod health_tests;
#[cfg(test)]
mod proportion_tests;
