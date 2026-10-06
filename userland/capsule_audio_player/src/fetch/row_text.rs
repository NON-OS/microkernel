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

//! What a row on the Downloads page says: how far it has got, how fast and
//! how long is left while it runs, where the file went, or why it stopped.

extern crate alloc;

use alloc::format;
use alloc::string::String;

use super::list::{Act, Row, State};

/// The row's line under its name.
pub fn status(row: &Row) -> String {
    match &row.state {
        State::Queued if row.done > 0 => format!("Waiting to go on from {}", size(row.done)),
        State::Queued => String::from("Waiting for the download before it"),
        State::Running => running(row),
        State::Stopping => String::from("Cancelling..."),
        State::Done(path) => format!("Saved to {path}"),
        State::Failed { why, .. } => String::from(*why),
        State::Cancelled if row.done > 0 => format!("Cancelled at {}; Resume goes on from there", size(row.done)),
        State::Cancelled => String::from("Cancelled"),
    }
}

fn running(row: &Row) -> String {
    let got = match row.total {
        Some(t) if t > 0 => format!("{} of {} ({}%)", size(row.done), size(t), row.done.saturating_mul(100) / t),
        _ if row.done == 0 => String::from("Connecting"),
        _ => size(row.done),
    };
    let mut line = got;
    if row.speed > 0 {
        line.push_str(", ");
        line.push_str(&rate(row.speed));
    }
    if let Some(s) = row.eta_secs() {
        line.push_str(", ");
        line.push_str(&left(s));
    }
    line
}

/// How far along, 0 to 1000, for the bar; None when the length is unknown.
pub fn permille(row: &Row) -> Option<u32> {
    let t = row.total.filter(|&t| t > 0)?;
    Some((row.done.min(t).saturating_mul(1000) / t) as u32)
}

/// A button's word.
pub fn label(act: Act) -> &'static str {
    match act {
        Act::Cancel => "Cancel",
        Act::Resume => "Resume",
        Act::Play => "Play",
        Act::Remove => "Remove",
    }
}

/// Bytes as a person reads them.
pub fn size(n: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * 1024;
    if n < KB {
        format!("{n} B")
    } else if n < MB {
        format!("{} KB", n / KB)
    } else {
        let tenths = n * 10 / MB;
        format!("{}.{} MB", tenths / 10, tenths % 10)
    }
}

/// A speed.
pub fn rate(bytes_per_sec: u64) -> String {
    format!("{}/s", size(bytes_per_sec))
}

/// Time left.
pub fn left(secs: u64) -> String {
    match secs {
        0..=59 => format!("{secs} s left"),
        60..=3599 => format!("{} min {} s left", secs / 60, secs % 60),
        _ => format!("{} h {} min left", secs / 3600, secs % 3600 / 60),
    }
}
