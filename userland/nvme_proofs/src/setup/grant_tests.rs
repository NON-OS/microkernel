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

//! A DMA grant shorter than asked, or not on a page, is refused and given
//! straight back: the controller would be named memory past what is ours,
//! or addresses the queue and PRP entries do not start at.

use nonos_libc::{calls, skew_grants};

use crate::dma::DmaRegion;
use crate::error::NvmeError;

fn refused(short: u64, offset: u64) {
    skew_grants(short, offset);
    let got = DmaRegion::map(7, 1, 8192).err();
    assert_eq!(got, Some(NvmeError::DmaGrantShort), "{short} short, {offset} in");
    let last = calls().into_iter().rev().take(2).collect::<Vec<_>>();
    assert_eq!((last[0].0, last[1].0), ("dma_unmap", "dma_map"), "given back");
    assert_eq!(last[0].1, last[1].1, "the grant it was given");
}

#[test]
fn a_short_grant_is_refused_and_given_back() {
    refused(1, 0);
    refused(4096, 0);
}

#[test]
fn a_grant_off_its_page_is_refused_and_given_back() {
    refused(0, 8);
    refused(0, 2048);
}

#[test]
fn a_whole_aligned_grant_is_taken() {
    skew_grants(0, 0);
    let Ok(r) = DmaRegion::map(7, 1, 8192) else { panic!("host memory") };
    assert_eq!(r.user_va() & 0xfff, 0);
    assert!(calls().iter().all(|c| c.0 != "dma_unmap"));
}
