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

//! Entries 2 and 3: what a publisher's wallet signs to bond a key and to
//! stand behind a release. The person reviews both in the wallet
//! before it asks, as for a transaction.

use super::chains::CHAINS;
use super::domains::BOND;
use super::entry::Entry;
use super::field::{Field, Kind};

const B32: Field = Field { kind: Kind::Bytes32, is_signer: false };

/// The wallet in any of its windows (src/userspace/capsule_wallet_nonos/spawn.rs).
const WALLET: &[&[u8]] = &[b"app.nonos_wallet", b"app.nonos_wallet.1", b"app.nonos_wallet.2"];

pub static PUBLISHER_BINDING: Entry = Entry {
    id: 2,
    type_string: "PublisherBinding(bytes32 publisherKey,bytes32 listingId,bytes32 bondId)",
    domain: &BOND,
    fields: &[B32; 3],
    callers: WALLET,
    // NoxPublisherBond is not deployed on either chain yet.
    contracts: [None; CHAINS.len()],
};

/// Escrow releases set packageHash to zero; others set chunkRoot to zero and
/// contentLen to 0.
pub static RELEASE_ATTEST: Entry = Entry {
    id: 3,
    type_string: "ReleaseAttest(bytes32 listingId,bytes32 releaseId,bytes32 bondId,\
                  bytes32 chunkRoot,uint64 contentLen,bytes32 packageHash)",
    domain: &BOND,
    fields: &[B32, B32, B32, B32, Field { kind: Kind::Uint(64), is_signer: false }, B32],
    callers: WALLET,
    contracts: [None; CHAINS.len()],
};
