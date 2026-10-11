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

//! A regulatory domain and what it allows on one channel. A beacon's Country
//! element may narrow it to the channels listed, never widen it: the element
//! is unauthenticated, so the domain the station was given stays the ceiling.

use super::band::{channel_mhz, Band};
use super::channel_set::ChannelSet;
use super::country::Country;
use super::permit::Permit;
use super::rule::{Rule, RuleFlags};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Domain {
    pub alpha2: [u8; 2],
    rules: &'static [Rule],
    listed: Option<ChannelSet>,
}

impl Domain {
    pub const fn new(alpha2: [u8; 2], rules: &'static [Rule]) -> Self {
        Self { alpha2, rules, listed: None }
    }

    /// The domain narrowed to the channels `country` lists.
    pub fn restrict(self, country: &Country) -> Self {
        let mut set = ChannelSet::EMPTY;
        for ch in 1..=255u8 {
            if country.channels.contains(ch) && self.listed.is_none_or(|l| l.contains(ch)) {
                set.insert(ch);
            }
        }
        Self { listed: Some(set), ..self }
    }

    /// The first rule a 20 MHz channel lies wholly inside, as cfg80211 picks.
    fn rule(&self, band: Band, channel: u8) -> Option<&Rule> {
        let mhz = channel_mhz(band, channel)?;
        self.rules.iter().find(|r| r.covers(mhz, 20))
    }

    pub fn permit(&self, band: Band, channel: u8) -> Permit {
        let Some(rule) = self.rule(band, channel) else {
            return Permit::Disabled;
        };
        let unlisted = band != Band::Ghz6 && self.listed.is_some_and(|l| !l.contains(channel));
        if unlisted || rule.flags.has(RuleFlags::NO_OFDM) {
            return Permit::Disabled;
        }
        if rule.flags.has(RuleFlags::NO_IR) {
            return Permit::Passive { dfs: rule.flags.has(RuleFlags::DFS) };
        }
        Permit::Active
    }

    /// The power ceiling on a channel, in dBm EIRP; `None` where disabled.
    pub fn max_eirp_dbm(&self, band: Band, channel: u8) -> Option<i8> {
        let open = self.permit(band, channel) != Permit::Disabled;
        self.rule(band, channel).filter(|_| open).map(|r| r.max_eirp_dbm)
    }
}
