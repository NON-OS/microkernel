// NONOS Operating System (AGPL-3.0-or-later)
//! Version 5's fields, as About reads them: the kernel's eight PCRs in its
//! order, then the event log, whose length must lie inside the document and
//! within the cap. Anything else is a layout About would be guessing at.

use crate::doc_build::{encoded, CHALLENGE, LOG, LOG_AT, PCRS_AT};
use crate::doc_parse::parse;
use crate::kernel_document::{log_fits, IOMMU_INTEL_VTD, MAX_LOG, QUOTED_PCRS};

fn good() -> Vec<u8> {
    encoded(&CHALLENGE, &[1, 2, 3], &[9; 72], 4, IOMMU_INTEL_VTD, true, 0)
}

#[test]
fn the_kernel_s_pcrs_and_log_are_read_past() {
    let v = good();
    assert_eq!(v[PCRS_AT], 8);
    assert_eq!(v[PCRS_AT + 1 + 33 * 3], QUOTED_PCRS[3]);
    let d = parse(&v, &CHALLENGE).expect("the kernel's document");
    assert_eq!((d.log_len as usize, d.attest_len, d.signature_len), (LOG.len(), 3, 72));
    assert_eq!(MAX_LOG, 1_048_576);
}

#[test]
fn another_count_or_order_of_pcrs_is_refused() {
    for (at, value) in [(PCRS_AT, 7), (PCRS_AT, 9), (PCRS_AT + 1, 1), (PCRS_AT + 1 + 33 * 7, 15)] {
        let mut v = good();
        v[at] = value;
        assert!(parse(&v, &CHALLENGE).is_none(), "byte {at} = {value}");
    }
}

#[test]
fn a_log_over_the_cap_or_past_the_end_is_refused() {
    for len in [LOG.len() as u32 + 1, MAX_LOG as u32 + 1, u32::MAX] {
        let mut v = good();
        v[LOG_AT..LOG_AT + 4].copy_from_slice(&len.to_be_bytes());
        assert!(parse(&v, &CHALLENGE).is_none(), "log length {len}");
    }
    let mut v = good();
    v[LOG_AT..LOG_AT + 4].copy_from_slice(&(LOG.len() as u32 - 1).to_be_bytes());
    assert!(parse(&v, &CHALLENGE).is_none(), "a log shorter than its bytes");
}

#[test]
fn a_whole_log_one_byte_over_the_cap_is_refused() {
    let v = good();
    let over = vec![0x4C; MAX_LOG + 1];
    let at_most = vec![0x4C; MAX_LOG];
    for (log, ok) in [(&over, false), (&at_most, true)] {
        let mut d = v[..LOG_AT].to_vec();
        d.extend((log.len() as u32).to_be_bytes());
        d.extend(log);
        d.extend(&v[LOG_AT + 4 + LOG.len()..]);
        assert_eq!(parse(&d, &CHALLENGE).is_some(), ok, "a log of {}", log.len());
    }
}

/// The kernel refuses, rather than cuts, a log over the cap (`log_fits`), so
/// every document it writes carries a log whose length About accepts.
#[test]
fn the_kernel_attests_a_log_up_to_the_cap_and_no_longer() {
    assert!(log_fits(&vec![0; MAX_LOG]) && !log_fits(&vec![0; MAX_LOG + 1]));
}
