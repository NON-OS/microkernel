// NONOS Operating System (AGPL-3.0-or-later)

//! A card that left the bus: the radio taken down and the stage said.

use super::connect::Session;
use super::radio::Radio;
use super::Stage;
use crate::status;

/// Once the card reads back all ones the radio is down for good: its link,
/// keys and session are dropped (there is no card to tell), the panel reads
/// `DeadMmio`, and the serve loop answers through a link that is always down.
pub(super) fn check_gone(radio: &mut Radio, session: &mut Option<Session>, stage: &mut Stage) {
    let Radio::Up(up) = radio else { return };
    if !up.link.card_gone() {
        return;
    }
    status::line(b"[rtl8821ce] the card reads back all ones: it left the bus, radio down\n");
    *session = None;
    *radio = Radio::Down;
    *stage = Stage::DeadMmio;
}
