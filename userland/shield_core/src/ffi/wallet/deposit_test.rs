/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::unwrap_used, clippy::indexing_slicing)]

use super::super::ticket::approve_calldata;
use crate::ffi::chain_types::format_wide;
use crate::ffi::evm::{parse_evm_address, to_hex};

/// Pinned against `cast calldata "approve(address,uint256)" <v2 pool> 1e18`.
#[test]
fn approve_matches_foundry() {
    let data = approve_calldata(1_000_000_000_000_000_000).unwrap();
    assert_eq!(
        to_hex(&data),
        "0x095ea7b3000000000000000000000000aee51e82965ec1ded870f3f4c248ad4addc3e1cb\
         0000000000000000000000000000000000000000000000000de0b6b3a7640000"
    );
}

#[test]
fn evm_address_round_trips_and_refuses_junk() {
    let text = "0xDaB92dCcEB48636e07848550858031cEaee3086f";
    let bytes = parse_evm_address(text).unwrap();
    assert_eq!(to_hex(&bytes), text.to_lowercase());
    for bad in [
        "",
        "0x",
        "DaB92dCcEB48636e07848550858031cEaee3086f",
        "0xZZ",
        &text[..41],
        "0x+aB92dCcEB48636e07848550858031cEaee3086f",
    ] {
        assert!(parse_evm_address(bad).is_err(), "{bad}");
    }
    assert!(parse_evm_address("0xDaB92dCcEB48636e07848550858031cEaee3086f00").is_err());
}

#[test]
fn wide_amounts_print_in_whole_tokens() {
    assert_eq!(format_wide(0), "0");
    assert_eq!(format_wide(1), "0.000000000000000001");
    assert_eq!(format_wide(25_500_000_000_000_000_000), "25.5");
    assert_eq!(format_wide(u128::MAX), "340282366920938463463.374607431768211455");
}
