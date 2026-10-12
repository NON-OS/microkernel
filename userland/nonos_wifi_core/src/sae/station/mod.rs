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

//! The station side of one SAE exchange with an access point (IEEE Std
//! 802.11-2020, 12.4.8): send a commit; answer an anti-clogging request by
//! sending it again with the token; on the AP's commit derive the keys and
//! send a confirm; on the AP's confirm check it and accept. The status code of
//! the AP's commit must say the same PWE method this station used (126 for
//! hash-to-element, 0 for hunting and pecking), a commit that only reflects
//! this station's own is dropped, and anything malformed or refused ends the
//! exchange with the reason kept for the panel.

mod accept;
mod check;
mod receive;
mod state;
mod types;

pub use state::SaeStation;
pub use types::{SaeFailure, SaeState, SaeStep};
