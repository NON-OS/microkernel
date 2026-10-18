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

//! The offer's buyer key (plan/29, the owner's rule 5), in the work that
//! asks for it, the answer that carries it and the offer kept: wiped when
//! each drops, and never printed: Debug names every field but that one.

use core::fmt;
use core::sync::atomic::{compiler_fence, Ordering};

use super::kept::KeptOffer;
use super::offer_work::OfferWork;
use super::want::Asked;

fn wipe(sk: &mut [u8; 32]) {
    sk.fill(0);
    compiler_fence(Ordering::SeqCst);
}

impl Drop for KeptOffer {
    fn drop(&mut self) {
        wipe(&mut self.buyer_sk);
    }
}

impl Drop for OfferWork {
    fn drop(&mut self) {
        wipe(&mut self.buyer_sk);
    }
}

impl Drop for Asked {
    fn drop(&mut self) {
        wipe(&mut self.buyer_sk);
    }
}

impl fmt::Debug for KeptOffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeptOffer")
            .field("chain_id", &self.chain_id)
            .field("listing_key", &self.listing_key)
            .field("release_key", &self.release_key)
            .field("chunk_root", &self.chunk_root)
            .field("content_len", &self.content_len)
            .field("buyer_sk", &"(redacted)")
            .field("made_at", &self.made_at)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for OfferWork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OfferWork")
            .field("chunk_root", &self.chunk_root)
            .field("content_len", &self.content_len)
            .field("buyer_sk", &"(redacted)")
            .field("made_at", &self.made_at)
            .finish_non_exhaustive()
    }
}
