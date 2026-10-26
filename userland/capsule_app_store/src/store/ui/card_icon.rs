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

//! The NONOS icon a catalogue app shows when the market serves no logo for
//! it (HANDOFF/catalogue/logos-licences.md): one of the desktop's own
//! glyphs, chosen by what the app is for. Drawn without asking the market,
//! so no logo ever holds up a row.

use nonos_toolkit::icons::IconId;

pub(super) fn of(listing: &[u8]) -> Option<IconId> {
    Some(match listing.strip_prefix(b"linux.")? {
        b"helix" | b"micro" | b"kakoune" | b"vim" | b"bat" => IconId::Editor,
        b"eza" | b"nnn" => IconId::Files,
        b"ncdu" => IconId::SettingsStorage,
        b"fzf" => IconId::SettingsSearch,
        b"git" | b"tig" | b"lazygit" => IconId::Terminal,
        b"age" | b"minisign" => IconId::GameLock,
        b"flac" | b"lame" => IconId::AudioPlayer,
        b"geth" => IconId::Wallet,
        b"profanity" => IconId::SettingsNetwork,
        _ => return None,
    })
}
