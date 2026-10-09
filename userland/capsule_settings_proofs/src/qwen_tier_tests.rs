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

//! Settings' "Qwen model" row is a choice among the 17 pinned tiers: stepping
//! visits each once and wraps, nothing set reads as the first (the one the
//! Terminal's qwen runs then), and a value is always a tier word the
//! Terminal and the policy record take.

use nonos_policy_proto::setup_record::{tier_ok, TIER_MAX};

use crate::qwen_tier::{shown, step};

#[test]
fn stepping_visits_every_tier_once_and_wraps() {
    let mut seen = Vec::new();
    let mut at: &[u8] = b"small";
    for _ in 0..17 {
        seen.push(at);
        at = step(at, 1);
    }
    assert_eq!(at, b"small", "seventeen steps come back round");
    seen.sort();
    seen.dedup();
    assert_eq!(seen.len(), 17);
    for tier in [&b"small"[..], b"qwen3-0.6b", b"qwen3-4b", b"coder-32b"] {
        assert!(seen.contains(&tier), "{tier:?}");
        assert_eq!(step(step(tier, 1), -1), tier);
    }
    assert_eq!(step(b"small", -1), b"coder-32b");
}

#[test]
fn nothing_set_reads_as_the_default_never_as_a_tier() {
    assert_eq!(shown(b""), b"Default (none chosen)");
    assert_eq!(shown(b"not-a-tier"), b"Default (none chosen)");
    assert_eq!(step(b"", 1), b"small");
    assert_eq!(step(b"", -1), b"coder-32b");
    assert_eq!(shown(b"qwen3-0.6b"), b"Qwen3 0.6B");
    assert_eq!(shown(b"qwen3-4b"), b"Qwen3 4B");
}

#[test]
fn every_tier_fits_the_policy_store() {
    let mut at: &[u8] = b"small";
    for _ in 0..17 {
        assert!(tier_ok(at), "{at:?}");
        assert!(at.len() <= TIER_MAX);
        at = step(at, 1);
    }
}
