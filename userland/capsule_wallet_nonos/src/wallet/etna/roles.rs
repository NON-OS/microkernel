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

//! Which face, size, tracking, case and colour each type role takes.

use super::face::Face;
use super::tokens::{TEXT, TEXT_3};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Role {
    ScreenLabel,
    Lead,
    RowName,
    RowValue,
    Fact,
    Button,
    ActionLabel,
    Status,
    Statement,
}

/// Face, size, tracking, capitals, colour.
pub(super) struct Spec(pub Face, pub f32, pub f32, pub bool, pub u32);

pub(super) fn spec(role: Role) -> Spec {
    match role {
        Role::ScreenLabel => Spec(Face::Mono, 12.0, 2.4, true, TEXT_3),
        Role::Lead | Role::RowName => Spec(Face::Sans, 14.0, 0.0, false, TEXT_3),
        Role::RowValue => Spec(Face::Mono, 13.0, 0.0, false, TEXT),
        Role::Fact => Spec(Face::Mono, 12.0, 0.0, false, TEXT),
        Role::Button => Spec(Face::Mono, 12.0, 1.6, true, TEXT),
        Role::ActionLabel => Spec(Face::SansMedium, 13.0, 0.0, false, TEXT),
        Role::Status => Spec(Face::Mono, 10.0, 0.6, false, TEXT_3),
        Role::Statement => Spec(Face::Mono, 21.0, 0.0, false, TEXT),
    }
}
