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
 * The image's own names (L52): the kernel's list, included by path, is held
 * to every Capsule.mk in the tree, so a capsule added or renamed without the
 * list fails here; and the rule that refuses a publisher's capsule a system
 * name, or a numbered instance of one, is run over the cases that matter.
 */

#[path = "../../../../src/services/registry/system_names.rs"]
mod system_names;
mod tests;
