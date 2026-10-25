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

//! Moving the background scan to its next channel, and what it has heard.

use super::sweep::Scanner;
use crate::regdom::{Band, Domain, ScanPlan, PASSIVE_DWELL_MS};
use crate::scan_list::ScanResults;
use crate::softmac::radio::{Env, Radio};

impl Scanner {
    /// The networks heard so far.
    pub fn results(&self) -> &ScanResults {
        &self.results
    }

    /// Start the sweep afresh after a session held the radio elsewhere.
    pub fn resume(&mut self) {
        (self.plan, self.at, self.hop_at_ms) = (None, None, 0);
    }

    /// Where `ssid` was last heard.
    pub fn hint(&self, ssid: &[u8]) -> Option<(Band, u8)> {
        self.hints.lookup(ssid)
    }

    pub(super) fn hop<R: Radio, E: Env>(&mut self, r: &mut R, env: &E, domain: Domain) {
        let (g2, g5) = r.bands();
        let next = self.plan.as_mut().and_then(Iterator::next);
        let step = match next {
            Some(s) => Some(s),
            None => {
                // A sweep ended: age the picture and start the next one.
                self.results.end_sweep();
                let mut plan = ScanPlan::new(domain, g2, g5);
                let first = plan.next();
                self.plan = Some(plan);
                first
            }
        };
        self.hop_at_ms = env.now_ms().saturating_add(u64::from(PASSIVE_DWELL_MS));
        self.at = step.filter(|s| r.tune(s.band, s.channel)).map(|s| (s.band, s.channel));
    }
}
