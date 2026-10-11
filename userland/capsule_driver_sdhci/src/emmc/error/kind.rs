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

//! The failures the host and card can report, and the result type.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmmcError {
    /// The broker refused a claim, a PCI command write, a mapping or a DMA
    /// region; the value is its errno.
    Broker(i64),
    /// The register window is too small for one SDHCI slot.
    Window,
    /// A generic SDHCI host whose slot is not an embedded one: it is a card
    /// reader or an SDIO slot, not the internal disk.
    NotEmbedded,
    /// The host does not offer ADMA2, the only DMA mode this driver drives.
    NoAdma,
    /// A software reset bit did not clear within 100 ms.
    HostReset,
    /// The internal clock did not report stable within 150 ms.
    ClockUnstable,
    /// The host does not hold SD Bus Power on after it was written.
    NoPower,
    /// The host and the card share no supply voltage.
    NoVoltage,
    /// CMD or DAT inhibit did not clear before a command could be sent.
    Inhibit(u8),
    /// The command got no response (Command Timeout Error alone).
    CmdTimeout(u8),
    /// The command phase failed; the value is the Error Interrupt Status.
    CmdError { cmd: u8, err: u16 },
    /// The data or busy phase failed; the value is the Error Interrupt Status.
    DataError { cmd: u8, err: u16 },
    /// Neither completion nor an error was posted before the deadline.
    NoCompletion(u8),
    /// No card answered CMD1.
    NoCard,
    /// The card answered CMD1 but stayed busy for a whole second.
    CardBusy,
    /// The card's R1 status carries error bits; the value is the status.
    Status { cmd: u8, status: u32 },
    /// The card is password locked.
    Locked,
    /// The card refused a SWITCH (CMD6) to this EXT_CSD byte.
    SwitchRefused(u8),
    /// The card is not in the state the step needs; the value is the status.
    BadState(u32),
    /// The card stayed busy past the deadline.
    CardStuck,
    /// The card reported no capacity, or one the driver cannot address.
    NoCapacity,
    /// No bus width read EXT_CSD back the same.
    BusWidth,
    /// A DMA region lies where the host cannot address it.
    DmaAddress,
    /// The sectors asked for lie outside the disk or the data buffer.
    OutOfRange,
}

pub type EmmcResult<T> = Result<T, EmmcError>;
