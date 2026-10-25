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

/*
 * Where the panic path is, from the kernel's own source: who takes it, what
 * a fault on its way out does, and that every fatal kernel fault and every
 * panic leave through it.
 */

#[cfg(test)]
mod branch_glue_tests;
#[cfg(test)]
mod glue_tests;
#[cfg(test)]
mod handlers_glue_tests;
#[cfg(test)]
mod helpers;
#[cfg(test)]
mod runtime_glue_tests;
#[path = "../../../../src/boot/panic/stage.rs"]
mod stage;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_orders;
#[cfg(test)]
mod tests_terminate;
