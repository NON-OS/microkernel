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

//! Every app built on app_skeleton reads its inbox with parse_delivery, and
//! any process can put bytes there. The decode takes any frame without a
//! panic and yields an event only from a frame long enough to hold one, with
//! the delivery magic and a known kind, with each field read from its own
//! offset. What it yields is then honoured only from the input router; that
//! check is a kernel lookup and is not run here.

mod bounds;
mod frames;
mod whole;
