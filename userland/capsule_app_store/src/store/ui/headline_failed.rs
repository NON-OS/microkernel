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

//! A failed install's headline, with which package and which check stopped
//! it when the installer said. The line is asked of the kernel as it is
//! painted, so this window keeps no copy of it.

use alloc::vec::Vec;

use nonos_libc::{mk_app_install_detail, MK_APP_INSTALL_DETAIL_MAX};
use nonos_market_proto::install_detail;

use crate::store::listing::Listing;
use crate::store::progress::Progress;

/// `line`, the reason's sentence, then ": " and the installer's line.
pub(super) fn with_detail(listing: &Listing, line: &[u8]) -> Vec<u8> {
    let mut out = line.to_vec();
    if !matches!(listing.progress, Progress::Failed(_)) {
        return out;
    }
    let mut buf = [0u8; MK_APP_INSTALL_DETAIL_MAX];
    let rc = mk_app_install_detail(&listing.id, &mut buf);
    if let Some(detail) = install_detail(&buf, rc) {
        out.extend_from_slice(b": ");
        out.extend_from_slice(detail.as_bytes());
    }
    out
}
