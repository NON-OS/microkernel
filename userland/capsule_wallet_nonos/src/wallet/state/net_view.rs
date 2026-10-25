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
//! The network as the window sees it: what the last probe found of the
//! route, the rails it may take, and the refresh under way.

use crate::wallet::net::last_read::LastRead;
use crate::wallet::net::step::Job;
use crate::wallet::net::NetStatus;

use super::rail::Rail;
use super::views::MAX_RAILS;

pub struct Network {
    /// What the last probe found of the route and the node behind it.
    pub link: NetStatus,
    pub rails: [Rail; MAX_RAILS],
    pub rail_count: usize,
    /// One when a refresh is wanted at the next idle tick rather than at the
    /// next interval: after a wallet arrives, or the network changes.
    pub probe_step: u8,
    /// The refresh under way, stepped a slice per tick so the window never
    /// waits on the network. None between refreshes.
    pub job: Option<Job>,
    /// The last whole read of mainnet (0) and Sepolia (1), for the screens.
    pub last_read: [Option<LastRead>; 2],
}

impl Network {
    pub fn new() -> Self {
        Self {
            link: super::default_net(),
            rails: [super::empty_rail::empty_rail(); MAX_RAILS],
            rail_count: 0,
            probe_step: 0,
            job: None,
            last_read: [None, None],
        }
    }
}
