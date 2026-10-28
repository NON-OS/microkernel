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
    // The SimpleX window. A stand-in until the core has a listing of its own:
    // the key of the terminal client's, linux.kali.nonos-simplex-chat in
    // nonos-data/market/catalogue, which market_proofs recomputes from it.
    ("app.simplex", SIMPLEX_CHAT),
];

/// The listing each capsule above may start by MkAppLaunch, by the id the
/// call names (app_launch_rule.rs). The kernel cannot read an installed
/// package's key at launch, so the id narrows the launch and the key binds
/// the serve; market_proofs holds each id to its row's key.
pub(crate) const LISTINGS: &[(&str, &str)] = &[
    // The key above is made from this id, under its catalogue publisher.
    ("app.simplex", "linux.kali.nonos-simplex-chat"),
];

const SIMPLEX_CHAT: [u8; 32] = [
    0x01, 0x24, 0xfa, 0x9a, 0xdc, 0x22, 0xb0, 0x9e, 0xfb, 0x6d, 0x59, 0x2a, 0x78, 0xc6, 0x4a, 0x7f,
    0xeb, 0x9e, 0x23, 0x45, 0x7c, 0x85, 0x36, 0x62, 0x3e, 0x50, 0x40, 0x2f, 0xbb, 0xdb, 0x76, 0x6e,
];
