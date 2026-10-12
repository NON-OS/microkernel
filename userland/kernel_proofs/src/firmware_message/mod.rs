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
 * The VideoCore property message the Raspberry Pi firmware is asked through.
 * The kernel's encoder and reader are included by path and checked against
 * the layout the firmware's mailbox documentation gives, with the firmware's
 * side played by the test.
 */

#[cfg(test)]
#[path = "../../../../src/arch/aarch64/firmware/message"]
mod message {
    pub(crate) mod answer;
    pub(crate) mod encode;
    pub(crate) mod tag;
}

#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod tests;
