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

//! The network a guest's stream leaves through follows the person's choice.
//!
//! It always went to the mixnet, so under the Anyone network every guest
//! connection failed and a guest's traffic left on a network nobody chose.
//! Held here over every default the policy store could hold, or none, with
//! each network up or down, through nonos_route_link's own pick: Anyone goes
//! to net.anon or nowhere, Nym and an unreadable default go to the mixnet,
//! and Direct goes to the mixnet too, since a guest never reaches the
//! network directly.

mod pick;
mod table;
