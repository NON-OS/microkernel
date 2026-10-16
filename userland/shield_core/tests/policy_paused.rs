//! The policy's paused() reply is an ABI bool read strictly: a garbled word is not "not paused".

use nox_shield_core::net::policy_paused::bool_word;

#[test]
fn paused_reads_as_zero_or_one_only() {
    let mut w = [0u8; 32];
    assert_eq!(bool_word(&w), Ok(false));
    w[31] = 1;
    assert_eq!(bool_word(&w), Ok(true));
    w[31] = 2;
    assert!(bool_word(&w).is_err());
    w[31] = 1;
    w[0] = 1;
    assert!(bool_word(&w).is_err());
    assert!(bool_word(&[0u8; 31]).is_err());
    assert!(bool_word(&[0u8; 64]).is_err());
}
