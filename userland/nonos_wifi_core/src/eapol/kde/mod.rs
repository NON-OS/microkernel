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

//! The Key Data field of an EAPOL-Key frame, once unwrapped: a run of
//! information elements and key data encapsulations (KDEs, IEEE Std
//! 802.11-2020, 12.7.2, Table 12-10). Message 3 carries the AP's RSNE (and
//! RSNXE) for the downgrade check, the GTK KDE and, with management frame
//! protection, the IGTK KDE; group message 1 carries the new GTK and IGTK. The
//! walk is bounded, refuses an element that runs past the buffer, and treats
//! the 0xDD-then-zeros tail as the padding the key wrap requires.

mod groups;
mod kinds;
mod parse;

pub use kinds::{GtkKde, IgtkKde, KeyData, TD_WPA3_PERSONAL};
pub use parse::parse_key_data;
