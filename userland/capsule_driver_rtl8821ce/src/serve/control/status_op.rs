// NONOS Operating System (AGPL-3.0-or-later)

//! The status op's reply: the bring-up stage and the data-path diagnostics.

use super::super::radio::Radio;
use super::super::Stage;
use super::status_body::status_body;
use super::WIFI_HDR;

/// The status reply: the stage, the link's counts, the efuse diagnosis and
/// the claim step that refused (`status_body`).
pub(super) fn status_reply(stage: Stage, radio: &Radio, out: &mut [u8]) -> Option<usize> {
    let stats = match radio {
        Radio::Up(up) => up.link.stats(),
        Radio::Down => crate::link::LinkStats::default(),
    };
    let claim = crate::setup::claim_note::read();
    let body = out.get_mut(WIFI_HDR..)?;
    let n = status_body(stage as u8, &stats, &crate::efuse::diag(), claim, body)?;
    Some(WIFI_HDR + n)
}
