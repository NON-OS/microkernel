//! The call against Foundry's encoder, the fee against the contract's rounding, the note kept.

use super::{account, hex, NOX, NOX_ASSET};
use nox_shield_core::discovery::confirm_deposit;
use nox_shield_core::wallet::{absorb_calldata, prepare_deposit, split_deposit, NATIVE_ASSET};

#[test]
fn the_call_is_byte_for_byte_what_foundry_encodes() {
    // cast calldata "absorb(uint64,uint256,bytes32)" 1 1000000000000000000 0x1122...1122
    let owner: [u8; 32] = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
        0x66, 0x77, 0x88, 0x99, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00,
        0x11, 0x22,
    ];
    let expected = "5cb945fa\
        0000000000000000000000000000000000000000000000000000000000000001\
        0000000000000000000000000000000000000000000000000de0b6b3a7640000\
        1122334455667788990011223344556677889900112233445566778899001122";
    assert_eq!(hex(&absorb_calldata(NOX_ASSET, u128::from(NOX), &owner)), expected);
}

#[test]
fn the_fee_is_the_contracts_rounding() {
    // fee = amount * bps / 10^4, rounded down, and the note holds the rest.
    assert_eq!(split_deposit(NOX, 25), Some((2_500_000_000_000_000, 997_500_000_000_000_000)));
    assert_eq!(
        split_deposit(399, 25),
        Some((0, 399)),
        "a fee below one base unit rounds to nothing"
    );
    assert_eq!(
        split_deposit(u64::MAX, 25).map(|(f, v)| f + v),
        Some(u64::MAX),
        "no overflow at the ceiling"
    );
    assert_eq!(
        split_deposit(1_000, 10_000),
        None,
        "a fee of everything is refused, as the pool does"
    );
}

#[test]
fn the_kept_note_is_the_leaf_the_pool_will_store() {
    let me = account().address();
    let req = prepare_deposit(&me, u128::from(NOX), NOX_ASSET, 25, 1).expect("deposit");
    assert_eq!(req.note.value, 997_500_000_000_000_000, "the note holds the net value");
    assert_eq!(req.value_wei, 0, "an ERC-20 deposit sends no value");
    assert_eq!(
        &req.calldata[36..68],
        &absorb_calldata(0, u128::from(NOX), &[0; 32])[36..68],
        "the gross amount"
    );
    assert_eq!(&req.calldata[68..], &req.owner_commit);
    // The watcher accepts it at the net value, beside the leaf the call creates.
    confirm_deposit(&req.note, 7, &req.commitment, req.note.value).expect("recognised");
    let native = prepare_deposit(&me, u128::from(NOX), NATIVE_ASSET, 25, 1).expect("native");
    assert_eq!(native.value_wei, u128::from(NOX), "a native deposit sends the amount as value");
}
