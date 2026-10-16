// NONOS Operating System (AGPL-3.0-or-later)
//! `nl`: each line numbered from one, two spaces, then the line as it was.

use crate::text::nl;

#[test]
fn every_line_is_numbered_from_one_and_kept_whole() {
    let input = vec![b"alpha".to_vec(), Vec::new(), b"  indented".to_vec()];
    let want: Vec<Vec<u8>> = vec![b"1  alpha".to_vec(), b"2  ".to_vec(), b"3    indented".to_vec()];
    assert_eq!(nl(input), want);
}

#[test]
fn the_number_grows_past_one_digit_without_padding() {
    let input: Vec<Vec<u8>> = (0..10).map(|i| vec![b'a' + i]).collect();
    let out = nl(input);
    assert_eq!(out[8], b"9  i".to_vec());
    assert_eq!(out[9], b"10  j".to_vec());
}

#[test]
fn no_input_numbers_nothing() {
    assert!(nl(Vec::new()).is_empty());
}
