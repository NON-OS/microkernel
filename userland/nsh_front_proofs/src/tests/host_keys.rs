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

//! The host keys' sealed layout and the label they are sealed under.

use crate::key_label::{root_label, BOUND_PREFIX, RECORD, ROOT_LABEL_LEN, ROOT_NAME};
use crate::key_layout::{join, split, ED_SEED_LEN, ML_PUBLIC_LEN, ML_SECRET_LEN, PLAIN_LEN};

#[test]
fn the_layout_is_the_three_parts_in_order_and_nothing_else() {
    assert_eq!(PLAIN_LEN, 32 + 4032 + 1952);
    let seed = [0x11u8; ED_SEED_LEN];
    let mut secret = [0u8; ML_SECRET_LEN];
    secret.iter_mut().enumerate().for_each(|(i, b)| *b = (i % 253) as u8);
    let mut public = [0u8; ML_PUBLIC_LEN];
    public.iter_mut().enumerate().for_each(|(i, b)| *b = (i % 241) as u8 ^ 0x5A);
    let mut plain = [0u8; PLAIN_LEN];
    join(&seed, &secret, &public, &mut plain);
    assert_eq!(&plain[..32], &seed[..]);
    assert_eq!(&plain[32..32 + ML_SECRET_LEN], &secret[..]);
    assert_eq!(&plain[32 + ML_SECRET_LEN..], &public[..]);
    let (s, k, p) = split(&plain);
    assert_eq!((s, &k[..], &p[..]), (&seed, &secret[..], &public[..]));
}

#[test]
fn the_label_is_bound_to_the_caller_and_within_the_kernels_bounds() {
    let label = root_label();
    assert_eq!(label[0], BOUND_PREFIX);
    assert_eq!(BOUND_PREFIX, 0x01, "the kernel's bound prefix (machine_key bound_label)");
    assert_eq!(&label[1..], ROOT_NAME);
    assert_eq!(label.len(), ROOT_LABEL_LEN);
    assert!(ROOT_NAME.len() <= 32, "the kernel takes 32 bytes after the prefix");
    assert_ne!(&label[..], b"nonos.vault.root.v1", "never the shared, unbound root");
    assert!(!RECORD.is_empty() && RECORD.len() <= 64, "nonos_vault's record bound");
}
