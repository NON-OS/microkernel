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

use core::sync::atomic::{AtomicU8, Ordering};

/// Which network the reader's requests leave through.
///
/// Nym is the default: a browser that has an anonymous route available and
/// takes the direct one without being asked publishes the address the route
/// exists to hide.
///
/// This is a choice, not a fallback. A request the chosen network cannot carry
/// fails rather than quietly leaving another way, because reverting on failure
/// would leak exactly when the network is worst. Changing the choice takes
/// effect on the next request; nothing already in flight changes route.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Network {
    Direct,
    Nym,
    Anyone,
}

impl Network {
    /// What the reader is shown, in the address bar and the settings panel.
    pub fn label(self) -> &'static str {
        match self {
            Network::Direct => "Direct, not anonymised",
            Network::Nym => "Nym mixnet",
            Network::Anyone => "Anyone network",
        }
    }
}

static CHOSEN: AtomicU8 = AtomicU8::new(1);

pub fn chosen() -> Network {
    match CHOSEN.load(Ordering::Relaxed) {
        0 => Network::Direct,
        2 => Network::Anyone,
        _ => Network::Nym,
    }
}

pub fn choose(n: Network) {
    let v = match n {
        Network::Direct => 0,
        Network::Nym => 1,
        Network::Anyone => 2,
    };
    CHOSEN.store(v, Ordering::Relaxed);
}
