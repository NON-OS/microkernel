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

//! The capsule's operator check over the trust a development image is built
//! with: its keys file names the dev tree's throwaway key instead of the one
//! this checkout tracks.

#[path = "../../../userland/capsule_market/src/bootstrap_trust/check.rs"]
mod check;
#[path = "dev_keys.rs"]
mod keys;

pub use check::is_trusted;
pub use keys::{DEV_OPERATOR, DEV_SEED};
