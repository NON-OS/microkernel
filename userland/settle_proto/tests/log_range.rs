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
//! plan/26's log cap per chain: 20 years of blocks from where a read starts
//! are paged, one more is EPROTO, and a head two hosts dispute is refused.

use nonos_nox_config::Chain;
use nonos_settle_proto::{blocks_in, confirmed_head, log_range, ENOENT, EPROTO, HEAD_SKEW_MS};
use nonos_settle_proto::{log_pages, SERVICE_MS};

const CHAINS: [Chain; 2] = [Chain::Mainnet, Chain::Sepolia];
const DEPLOY: u64 = 7_000_000;

#[test]
fn the_cap_is_twenty_years_of_each_chains_blocks() {
    assert_eq!(SERVICE_MS, 20 * 36_525 * 86_400_000 / 100, "20 years of 365.25 days");
    assert_eq!(blocks_in(Chain::Mainnet, SERVICE_MS), 52_596_000, "12 s blocks");
}

#[test]
fn a_head_twenty_years_out_is_paged() {
    for chain in CHAINS {
        let head = DEPLOY + blocks_in(chain, SERVICE_MS);
        let pages: Vec<_> = log_pages(chain.id(), DEPLOY, head).expect("in the cap").collect();
        assert_eq!(pages.first().map(|p| p.0), Some(DEPLOY), "{chain:?}");
        assert_eq!(pages.last().map(|p| p.1), Some(head), "{chain:?}");
        assert_eq!(pages.len(), 5_260, "{chain:?}: 10 000 blocks a page");
    }
}

#[test]
fn a_head_past_the_cap_is_eproto() {
    for chain in CHAINS {
        let head = DEPLOY + blocks_in(chain, SERVICE_MS) + 1;
        assert_eq!(log_range(chain.id(), DEPLOY, head), Err(EPROTO), "{chain:?}");
        assert!(log_pages(chain.id(), DEPLOY, u64::MAX).is_err(), "{chain:?}");
    }
}

#[test]
fn from_the_mark_a_range_stays_small_and_a_mark_past_the_head_pages_nothing() {
    for chain in CHAINS {
        assert_eq!(log_pages(chain.id(), DEPLOY + 1, DEPLOY + 25_000).unwrap().count(), 3);
        assert_eq!(log_pages(chain.id(), DEPLOY + 9, DEPLOY).unwrap().count(), 0);
    }
    assert_eq!(log_range(5, 0, 1), Err(ENOENT), "a chain the config lacks");
}

#[test]
fn a_head_that_jumps_is_refused() {
    for chain in CHAINS {
        let (skew, h) = (blocks_in(chain, HEAD_SKEW_MS), DEPLOY + 1_000);
        assert_eq!(confirmed_head(chain, h + skew, h), Some(h), "{chain:?}: the lower one");
        assert_eq!(confirmed_head(chain, h, h + skew + 1), None, "{chain:?}");
        assert_eq!(confirmed_head(chain, u64::MAX, h), None, "{chain:?}: one host lying");
    }
}
