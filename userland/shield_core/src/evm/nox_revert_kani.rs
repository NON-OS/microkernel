//! Proved for every revert a node could hand back, up to a hundred bytes.

use super::why;

#[kani::proof]
#[kani::unwind(102)]
fn any_revert_data_reads_as_a_sentence_without_panicking() {
    let bytes: [u8; 100] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= bytes.len());
    assert!(!why(&bytes[..len]).is_empty());
}
