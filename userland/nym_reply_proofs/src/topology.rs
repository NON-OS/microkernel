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

//! The topology answer a session open asks for. The capsule's comes from the
//! directory it fetched; here a usable one is always in hand, so the proofs
//! reach the table's own rules.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TopologyStatus {
    Missing,
    Ready,
    Expired,
}

pub fn status() -> TopologyStatus {
    TopologyStatus::Ready
}
