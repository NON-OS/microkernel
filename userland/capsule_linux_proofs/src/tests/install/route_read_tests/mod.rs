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

//! Reading a mirror's reply through the network the person chose: the
//! reader waits long for the first byte and per quiet stretch after it,
//! stops once the reply is whole or the far end has finished, keeps what
//! came before a break, and refuses a reply larger than anything it fetches.
//! The stream is scripted here; the capsule reads a `RouteStream`.

mod limits;
mod pieces;
mod script;
