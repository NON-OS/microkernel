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

//! The attack suite's capsule: it tries what an attacker would, each attempt
//! one it holds no right to, and prints the kernel's answer to each on the
//! serial line for nonos-ci/attack_suite_check.py to judge. Development
//! images only (the selftest profile).

#[cfg(feature = "nonos-release")]
compile_error!("the attack suite's capsule is a development test and never ships in a release");

mod embed;
mod spawn;

pub use spawn::run;
