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

//! What the toolkit service's OP_COMPONENT_RENDER paints with once its
//! surface is attached: the panel fill from component_dispatch's paint, and
//! the button and label components.

#[path = "../../toolkit/src/components/button.rs"]
pub mod button;
#[path = "../../toolkit/src/component_dispatch/paint/fill_rect.rs"]
pub mod fill_rect;
#[path = "../../toolkit/src/components/label.rs"]
pub mod label;
