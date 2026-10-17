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
//! The host's side of a virtio-gpu, as a model answering the control queue
//! on the driver's own thread, written from the test's own reading
//! of the 2D commands as recalled from the virtio specification (not
//! verified against it): resources with their own pixels and backing,
//! and what each scanout shows.

pub mod apply;
pub mod fresh;
pub mod pixels;
pub mod serve;
pub mod state;
pub mod words;
