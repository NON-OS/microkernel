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

//! An authenticated link to one relay.

mod bind;
mod certs;
mod constants;
mod ed_cert;
mod frames;
mod held;
mod netinfo;
mod open;
mod pump;
mod session;
mod sni;
mod socket;
mod timing;
mod tls_fault;
mod versions;
mod versions_read;

pub use open::open;
pub use session::{Link, LinkError};
