// NONOS Operating System (AGPL-3.0-or-later)

//! The status op's reply: the bring-up stage and the data-path diagnostics.

use super::super::radio::Radio;
use super::super::Stage;
use super::status_body::status_body;
use super::WIFI_HDR;

/// The status reply: the stage, the link's counts and the efuse diagnosis
/// (`status_body`).
pub(super) fn status_reply(stage: Stage, radio: &Radio, out: &mut [u8]) -> Option<usize> {
    let stats = match radio {
        Radio::Up(up) => up.link.stats(),
        Radio::Down => crate::link::LinkStats::default(),
    };
    let n = status_body(stage as u8, &stats, &crate::efuse::diag(), out.get_mut(WIFI_HDR..)?)?;
    Some(WIFI_HDR + n)
}
