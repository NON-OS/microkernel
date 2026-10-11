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

//! Pipes and the transcoders that feed them on Gen9 (Skylake to Comet
//! Lake): three pipes, transcoders A to C each bound to its pipe, and the
//! eDP transcoder, which feeds whichever pipe TRANS_DDI_FUNC_CTL_EDP
//! selects (SKL PRM Vol 2c, TRANS_DDI_FUNC_CTL, eDP Input Select).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pipe {
    A,
    B,
    C,
}

impl Pipe {
    pub const ALL: [Pipe; 3] = [Pipe::A, Pipe::B, Pipe::C];

    pub fn index(self) -> u32 {
        self as u32
    }

    /// The wire's output number: pipe A is output 0.
    pub fn from_output(output: u16) -> Option<Pipe> {
        Pipe::ALL.get(output as usize).copied()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transcoder {
    A,
    B,
    C,
    Edp,
}

impl Transcoder {
    /// The transcoder that bears a pipe's own letter.
    pub fn of(pipe: Pipe) -> Transcoder {
        match pipe {
            Pipe::A => Transcoder::A,
            Pipe::B => Transcoder::B,
            Pipe::C => Transcoder::C,
        }
    }

    /// The pipe the eDP transcoder feeds, from eDP Input Select (bits
    /// 14:12): 000 and 100 pipe A, 101 pipe B, 110 pipe C.
    pub fn edp_pipe(func_ctl: u32) -> Option<Pipe> {
        match (func_ctl >> 12) & 7 {
            0 | 4 => Some(Pipe::A),
            5 => Some(Pipe::B),
            6 => Some(Pipe::C),
            _ => None,
        }
    }
}
