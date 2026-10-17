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
 * The broker's class table included by path (L79), under the module path
 * the kernel gives it, so its restricted ids are in reach here. find counts
 * a PCI function as a display by the class id its class code maps to, so
 * the map is checked over every code there is.
 */

#![allow(dead_code)]

#[path = "../../../../../src/hardware/broker/class.rs"]
pub mod class;
mod class_tests;
