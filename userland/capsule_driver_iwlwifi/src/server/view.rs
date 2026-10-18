// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! What the control family is told about the radio.

use super::control::{Heard, View};
use super::radio::Radio;

pub(super) fn view(radio: &Radio) -> View<'_> {
    let (stage, step, detail) = radio.stage();
    let (sweeps, stalls, frames) = radio.counts();
    let heard = radio.scanning().then_some(Heard {
        list: &radio.results,
        sweeps,
        stalls,
        frames,
        beacons: radio.beacons,
    });
    View { stage, step, detail, heard }
}
