// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The exchange's counts for the connect reply, and the MLME state as the
//! reply's progress byte.

use nonos_wifi_core::mlme::MlmeState;

/// The exchange's counts, for the connect reply.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Progress {
    pub sent: u32,
    pub recv: u32,
    pub data: u32,
    pub eapol: u32,
    pub deauth: u32,
    pub to_us: u32,
    /// The first unanswered data frame's frame control (low 16 bits) and
    /// length (high 16), for a diagnosis of what arrived instead of EAPOL.
    pub probe: u32,
    /// Frames the queues would not take.
    pub refused: u32,
    /// How far the MLME got: 0 scanning, 1 authenticating, 2 associating,
    /// 3 four-way, 4 connected, 5 failed.
    pub state: u8,
}

/// The MLME's state as the connect reply's progress byte.
pub fn state_code(s: MlmeState) -> u8 {
    match s {
        MlmeState::Scanning => 0,
        MlmeState::Authenticating => 1,
        MlmeState::Associating => 2,
        MlmeState::FourWay => 3,
        MlmeState::Connected => 4,
        MlmeState::Failed => 5,
    }
}
