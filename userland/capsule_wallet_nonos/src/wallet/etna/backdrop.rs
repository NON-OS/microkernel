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

//! The section photographs: Etna in eruption, recoloured to ink, deep teal
//! and cyan. Ten sections, eight photographs; see assets/etna/index.txt.
//! Photo: gnuckx, CC BY 2.0, recoloured by NONOS (assets/etna/CREDITS.txt).

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Backdrop {
    Welcome,
    Home,
    Send,
    Receive,
    Deposit,
    Withdraw,
    Proving,
    History,
    Settings,
    Backup,
}

const WELCOME: &[u8] = include_bytes!("../../../../assets/etna/etna-387f21b9dbd2.png");
const HOME: &[u8] = include_bytes!("../../../../assets/etna/etna-90380f7829df.png");
const SEND: &[u8] = include_bytes!("../../../../assets/etna/etna-4bb30d031341.png");
const RECEIVE: &[u8] = include_bytes!("../../../../assets/etna/etna-fcb6fdda9325.png");
const DEPOSIT: &[u8] = include_bytes!("../../../../assets/etna/etna-460f26b9c468.png");
const WITHDRAW: &[u8] = include_bytes!("../../../../assets/etna/etna-a0754afa8556.png");
const PROVING: &[u8] = include_bytes!("../../../../assets/etna/etna-0b0046f5f585.png");
const HISTORY: &[u8] = include_bytes!("../../../../assets/etna/etna-32f5aa765d0e.png");

impl Backdrop {
    /// The encoded photograph; sections that share one share the bytes.
    pub fn png(self) -> &'static [u8] {
        match self {
            Backdrop::Welcome => WELCOME,
            Backdrop::Home => HOME,
            Backdrop::Send | Backdrop::Backup => SEND,
            Backdrop::Receive | Backdrop::Settings => RECEIVE,
            Backdrop::Deposit => DEPOSIT,
            Backdrop::Withdraw => WITHDRAW,
            Backdrop::Proving => PROVING,
            Backdrop::History => HISTORY,
        }
    }
}

/// The credit every place the photographs appear must carry.
pub const CREDIT: &str = "Photo: gnuckx, CC BY 2.0, recoloured by N\u{d8}NOS";
