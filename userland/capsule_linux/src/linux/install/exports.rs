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

//! What the install role gives the rest of the personality, named at `crate::linux::install` by mod.rs.

pub use super::apps_run::launch;
pub use super::family::install;
pub use super::market::arg::{market_arg, NotMarket, MARKET_LEN};
pub use super::market::install_from;
pub use super::record::program::recorded;
pub use super::remove::package::uninstall;
pub use super::why::Why;
