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

//! Every controller started, served as one host. Field order is drop
//! order: the controllers' schedules go before the claims they live on.

use super::claim::Claim;
use super::discover::{find_all, Found};
use super::sequence::{run, Driver};
use crate::platform::{Region, Registers, Uptime};
use crate::server::context::Ctx;
use crate::server::many::{Front, MAX_HC};

pub struct Host {
    pub front: Front<Ctx<Registers, Uptime, Region>>,
    pub found: [Option<Found>; MAX_HC],
    claims: [Option<Claim>; MAX_HC],
}

impl Host {
    /// `first` started, then every other controller found that starts.
    pub fn new(first: Driver) -> Self {
        let mut host =
            Host { front: Front::new(), found: [None; MAX_HC], claims: Default::default() };
        let first_id = first.found.device_id;
        host.add(first);
        for dev in find_all().into_iter().flatten().filter(|d| d.device_id != first_id) {
            if let Ok(d) = run(dev) {
                host.add(d);
            }
        }
        host
    }

    fn add(&mut self, d: Driver) {
        let Some(i) = self.claims.iter().position(|c| c.is_none()) else { return };
        if self.front.add(d.cx) {
            (self.found[i], self.claims[i]) = (Some(d.found), Some(d.claim));
        }
    }
}
