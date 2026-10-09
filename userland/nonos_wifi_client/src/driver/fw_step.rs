/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/// What an RTL8821CE firmware failure code (the step byte its status reply
/// carries after the data-path counters) means, for the panel line that says
/// where the firmware load stopped. `None` for no failure or a code this build
/// does not know.
pub fn firmware_step_text(code: u8) -> Option<&'static str> {
    Some(match code {
        1 => "the embedded image is not 8821C firmware",
        2 => "no DMA memory for the staging buffer",
        3 => "no DMA memory for the beacon ring",
        4 => "DMA memory came back above 4 GB",
        5 => "the card never fetched the staged chunk",
        6 => "the card's DMA copy never finished",
        7 => "a section failed its checksum",
        8 => "the card did not confirm the checksums",
        9 => "the card's CPU never reported ready",
        10 => "the card's transmit engines did not start",
        11 => "the coexistence port never answered",
        _ => return None,
    })
}
