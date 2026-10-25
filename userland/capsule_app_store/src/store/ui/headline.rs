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

//! The selected listing's headline: what happened once the person asked,
//! else the market's verdict, and the colour it is said in.

use alloc::vec::Vec;

use nonos_market_proto::installed_line;

use crate::store::listing::Listing;
use crate::store::market::Readiness;
use crate::store::next_step::refused_uninstall;
use crate::store::progress::Progress;
use crate::store::progress_text::waiting_behind;
use crate::store::reasons::reason;
use crate::store::route_offer::progress_line;
use crate::store::state::State;
use crate::store::theme::{ACCENT, DANGER, MUTED, OK};
use crate::store::verdict::Verdict;

pub(super) fn headline(state: &State, listing: &Listing, r: &Readiness) -> (Vec<u8>, u32) {
    let v = Verdict::of(r);
    let checking = listing.known.recheck_at.is_some();
    let hue = match v {
        Verdict::Ready => OK,
        Verdict::Installed => ACCENT,
        Verdict::Checking if checking => ACCENT,
        Verdict::Checking | Verdict::Blocked => DANGER,
    };
    // A blocked listing says which gate stopped it (B9).
    let said = match v {
        Verdict::Checking | Verdict::Blocked => reason(r, checking).unwrap_or(v.sentence()),
        _ => v.sentence(),
    };
    // Once the person has asked, what happened is the headline. Installed
    // says where it is held (`installed_line`): never "kept" for what goes.
    let (line, tone) = match listing.progress {
        Progress::Installed => (installed_line(&listing.id).as_bytes(), OK),
        p if refused_uninstall(p, listing.removing) => {
            (&b"The system would not start the uninstall; press u to ask again"[..], DANGER)
        }
        p => p.sentence().unwrap_or((said, hue)),
    };
    // While a tier's model comes, the fetcher's line is the headline.
    let fetching = state.fetch_of(listing).map(|s| progress_line(&s));
    /* Queued behind another install: which, and how far it has got. */
    let behind = (listing.progress == Progress::Queued)
        .then(|| state.ahead_of(listing))
        .flatten()
        .map(|ahead| {
            let how = state.fetch_of(ahead).map(|s| progress_line(&s));
            waiting_behind(&ahead.name, how.as_deref())
        });
    match (fetching, behind) {
        (Some(said), _) => (said.into_bytes(), ACCENT),
        (None, Some(said)) => (said, MUTED),
        (None, None) => (line.to_vec(), tone),
    }
}
