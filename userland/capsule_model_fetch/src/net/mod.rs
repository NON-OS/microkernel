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
 * How the fetcher reaches a mirror: the network the machine runs, chosen
 * by which network capsules are up, as the browser and the Terminal choose,
 * and never a quiet fallback to a direct connection.
 */

mod anon;
mod anon_call;
mod anon_io;
mod link;
mod route;
mod socks;
mod socks_call;
mod socks_io;

pub use link::Link;
pub use route::Route;
