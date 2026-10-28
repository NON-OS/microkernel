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

//! What follows the policy and its keys on the install word, read whole
//! before anything is fetched: the listed arguments (nonos_linux_policy::args)
//! and the extra packages the closure adds (nonos_linux_policy::with), each
//! canonical or absent. The extras are resolved through the signed index like
//! any need; the market's pin holds the listed package alone.

use alloc::vec::Vec;

use nonos_linux_policy::args::check;
use nonos_linux_policy::with::{self, split};

use super::arg::Market;

impl Market {
    /// Its arguments, when there are none or they are canonical, and so are its extras.
    pub fn listed(&self) -> Result<&[u8], &'static str> {
        let (args, _) = split(&self.args);
        match args.is_empty() || check(args).is_ok() {
            true => self.extras().map(|_| args),
            false => Err("its listed arguments are malformed"),
        }
    }

    /// Its extra packages: none, or at most with::MAX_NAMES names, each by
    /// the package rule and none twice.
    pub fn extras(&self) -> Result<Vec<&str>, &'static str> {
        let (_, extras) = split(&self.args);
        if !extras.is_empty() && with::check(extras).is_err() {
            return Err("its extra packages are malformed");
        }
        // The package rule admits ASCII alone, so each name is a str.
        Ok(with::names(extras).filter_map(|n| core::str::from_utf8(n).ok()).collect())
    }

    /// Both, read whole.
    pub fn tail(&self) -> Result<(&[u8], Vec<&str>), &'static str> {
        Ok((self.listed()?, self.extras()?))
    }
}
