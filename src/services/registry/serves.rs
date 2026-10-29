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

//! Which capsule a Linux run may serve its guest's one listener to, by the
//! listing the run was bound to (SERVE_ONE, serve_rule.rs).
//!
//! The run role names the capsule and the listing key it bound and checked
//! at start. The kernel serves the listener only for a pair listed here, so
//! another listing that claims the capsule is refused, and the capsule never
//! reaches an impostor. The table lives in the kernel image, so it is
//! measured with it, as the peer lists are (peers.rs); a signed listing field
//! may replace it.

/// Capsule, then the listing key a run serving it was bound to:
/// keccak256(LISTING_TAG, publisher key, keccak256(listing id)), the id a
/// market policy carries at byte 56.
pub(crate) const SERVES: &[(&str, [u8; 32])] = &[
    // The SimpleX window: its core's own listing, not the terminal client's,
    // linux.kali.nonos-simplex-core in nonos-data/market/catalogue, which
    // market_proofs recomputes it from.
    ("app.simplex", SIMPLEX_CORE),
];

/// The listing each capsule above may start by MkAppLaunch, by the id the
/// call names (app_launch_rule.rs). The kernel cannot read an installed
/// package's key at launch, so the id narrows the launch and the key binds
/// the serve; market_proofs holds each id to its row's key.
pub(crate) const LISTINGS: &[(&str, &str)] = &[
    // The key above is made from this id, under its catalogue publisher.
    ("app.simplex", "linux.kali.nonos-simplex-core"),
];

/// The one word each capsule above may give the run it launches, by its
/// prefix (app_launch_word.rs): the core's profile name, on its first start.
pub(crate) const WORDS: &[(&str, &str)] = &[("app.simplex", "--user-display-name=")];

const SIMPLEX_CORE: [u8; 32] = [
    0x89, 0x00, 0xcc, 0xe8, 0x3c, 0xfd, 0xfe, 0xc6, 0xa0, 0x7a, 0xa4, 0x78, 0x18, 0x96, 0xc9, 0x12,
    0xb3, 0x11, 0xd6, 0x7f, 0x74, 0x35, 0x03, 0x5a, 0xa6, 0x0e, 0xac, 0xc8, 0x5b, 0x0a, 0x59, 0xa4,
];
