//! NOX's state for a swap, from the ten answers `reads::calls` asks for it.

use super::quote::NoxState;
use crate::error::NetError;
use crate::evm::network::Chain;
use crate::evm::reply::Answer;

pub(super) fn state(
    chain: &Chain,
    a: &[Answer],
) -> Result<Result<NoxState, &'static str>, NetError> {
    let data = |i: usize| a.get(i).ok_or(NetError::ReplyShape)?.data();
    let flag = |i: usize| -> Result<bool, NetError> { Ok(data(i)?.last() == Some(&1)) };
    let number = |i: usize| a.get(i).ok_or(NetError::ReplyShape)?.word();
    let slot = data(0)?;
    let implementation = slot.get(12..32).ok_or(NetError::ReplyShape)?;
    if !chain.nox_implementations.iter().any(|k| k.as_slice() == implementation) {
        return Ok(Err(
            "NOX's contract changed after this app was built. Update the app to swap NOX.",
        ));
    }
    if flag(1)? {
        return Ok(Err("NOX transfers are paused."));
    }
    if flag(2)? {
        return Ok(Err("The NOX token blocks this account."));
    }
    let fees = data(3)?;
    let bps = |k: usize| -> Result<u16, NetError> {
        u16::try_from(super::parse::word(&fees, k)?).map_err(|_| NetError::ReplyShape)
    };
    let fee_sale =
        if flag(5)? && flag(8)? { Some((number(6)?, number(7)?, number(9)?)) } else { None };
    Ok(Ok(NoxState {
        token: chain.nox_token,
        buy_bps: bps(0)?,
        sell_bps: bps(1)?,
        burn_share_bps: bps(3)?,
        exempt: flag(4)?,
        fee_sale,
    }))
}
