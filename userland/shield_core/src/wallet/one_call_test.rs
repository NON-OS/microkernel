// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::arithmetic_side_effects)]

use super::one_call;

/// A shape A package whose body bytes each carry their own position, so every slice of the output
/// can be traced back to where the cut took it from.
fn package(body: usize) -> Vec<u8> {
    let mut out = b"NOXP".to_vec();
    out.extend_from_slice(&[7, 0, 1, 0]);
    out.extend_from_slice(&[0xad, 0xd1, 0x8d, 0xbb]);
    out.extend_from_slice(&[0x11; 28]);
    out.extend((0..body).map(|i| (i % 251) as u8));
    out
}

fn word(out: &[u8], at: usize) -> usize {
    usize::from_be_bytes(out[at + 24..at + 32].try_into().expect("a word"))
}

fn part(out: &[u8], offset_at: usize) -> &[u8] {
    let start = word(out, offset_at);
    let len = word(out, start);
    &out[start + 32..start + 32 + len]
}

/// The sizes a live v2 settlement carries, read from `0x3244a854…6e5e` on Sepolia: a head of
/// 5,808 bytes, claims of 956, and the parameter id in the first trailing word.
#[test]
fn a_package_cuts_as_the_pool_reads_it() {
    let body = 95_976 - 40;
    let pkg = package(body);
    let out = one_call(&pkg).expect("a shape A package");
    assert_eq!(&out[..8], &[0x43, 0x63, 0x09, 0xdb, 0x1d, 0xfe, 0xc7, 0xfc]);
    assert_eq!(&out[128..160], &pkg[8..40]);
    assert_eq!(word(&out, 160), 0);
    let (head, claims, queries) = (part(&out, 32), part(&out, 64), part(&out, 96));
    assert_eq!((head.len(), claims.len()), (5_808, 956));
    assert_eq!(head.len() + claims.len() + queries.len(), body);
    let b = &pkg[40..];
    let (f, r) = (5_716 + 19 * 384, 5_716 + 19 * 384 + 92 + 19 * 368);
    assert_eq!(&head[..5_716], &b[..5_716]);
    assert_eq!(&head[5_716..], &b[f..f + 92]);
    assert_eq!(claims, &b[r..r + 956]);
    assert_eq!(&queries[..19 * 384], &b[5_716..f]);
    assert_eq!(&queries[19 * 384..19 * 384 + 19 * 368], &b[f + 92..r]);
    assert_eq!(&queries[19 * 384 + 19 * 368..], &b[r + 956..]);
}

#[test]
fn any_other_package_is_refused() {
    let good = package(95_936);
    let mut other_shape = good.clone();
    other_shape[8] = 0x94;
    let mut other_format = good.clone();
    other_format[4] = 6;
    assert!(one_call(&other_shape).is_none());
    assert!(one_call(&other_format).is_none());
    assert!(one_call(&good[..20_000]).is_none());
}
