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

//! The claim's own types and stage rule, included by path, so the model
//! takes its entry through the stages the kernel does.

#[allow(clippy::duplicate_mod)]
#[path = "../../../../src/hardware/broker/claim/stage_rule.rs"]
pub(super) mod stage_rule;
#[allow(clippy::duplicate_mod)]
#[path = "../../../../src/hardware/broker/claim/types.rs"]
pub(super) mod types;
