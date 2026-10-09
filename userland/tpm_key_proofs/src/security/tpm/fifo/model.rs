// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A FIFO register file of locality 0, with the states the TCG PC Client
//! Platform TPM Profile gives it, and switches for the ways a part can
//! misbehave. Anything the driver does that a real part would not accept
//! is recorded as a violation rather than silently tolerated.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Ready,
    Reception,
    Execution,
    Completion,
}

pub struct Model {
    pub state: State,
    pub active: bool,
    /// Whether a locality request is granted.
    pub grant: bool,
    /// The burst count offered while there is room or data.
    pub burst: usize,
    /// Accesses the driver may still make on the last burst count it read.
    pub budget: usize,
    pub received: Vec<u8>,
    /// The last command the part ran.
    pub ran: Vec<u8>,
    /// What execution answers, header included.
    pub response: Vec<u8>,
    pub pos: usize,
    /// Status polls in Execution before the response is ready.
    pub exec_polls: u64,
    /// The command length the part parses, when not the header's own.
    pub parsed_len: Option<usize>,
    pub clock: u64,
    pub violations: Vec<&'static str>,
    pub commands: usize,
}

impl Model {
    pub fn new(response: Vec<u8>) -> Self {
        Self {
            state: State::Idle,
            active: false,
            grant: true,
            burst: 3,
            budget: 0,
            received: Vec::new(),
            ran: Vec::new(),
            response,
            pos: 0,
            exec_polls: 4,
            parsed_len: None,
            clock: 0,
            violations: Vec::new(),
            commands: 0,
        }
    }
}
