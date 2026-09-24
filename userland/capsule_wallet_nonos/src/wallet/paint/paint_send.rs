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

use super::scale::{self, space};

// The card, and the rhythm inside it.
const CARD_TOP: u32 = 146;
const CARD_H: u32 = 470;
// A seventeen-pixel line box needs more than twenty pixels of clearance, which
// is what the hand-picked offsets gave it.
const LABEL_DROP: u32 = 28;
const FIELD_H: u32 = 40;
const FEE_H: u32 = 42;
const WARN_H: u32 = 40;
use nonos_app_skeleton::PaintBuffer;

use super::ui;
use crate::wallet::state::{State, SEND_FIELD_AMOUNT, SEND_FIELD_TO};
use crate::wallet::theme::{ACCENT, AMBER, DIM, FG, INK, LINE2, MUTED, PANEL_2};

pub fn paint_send(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    let lw = 640u32;
    let ix = cx + 20;
    let iw = lw - 40;
    ui::card(fb, cx, CARD_TOP, lw, CARD_H);

    // Every row below is derived from the spacing unit rather than typed in by
    // eye. The old numbers put each label twenty pixels above its field, which
    // was right for the thirteen-point text they were chosen for and three
    // pixels short of the seventeen the font actually draws, so all four pairs
    // overlapped. Deriving them means the rhythm survives the next type change
    // instead of quietly colliding again.
    let label_to = CARD_TOP + space(4);
    let field_to = label_to + LABEL_DROP;
    let hint_to = field_to + FIELD_H + space(2);
    let label_amount = hint_to + space(9);
    let field_amount = label_amount + LABEL_DROP;
    let label_fee = field_amount + FIELD_H + space(5);
    let box_fee = label_fee + LABEL_DROP;
    let box_warn = box_fee + FEE_H + space(4);
    let button = box_warn + WARN_H + space(5);

    // Asset toggle: send ETH (a value transfer) or NOX (an ERC-20 transfer).
    asset_tab(fb, ix + iw - 156, 152, "ETH", state.send_token == 0);
    asset_tab(fb, ix + iw - 76, 152, "NOX", state.send_token == 1);

    // The recipient exactly as typed, prefixed 0x, or an empty-field prompt.
    let _ = fb.text_ttf(ix as i32, label_to as i32, "RECIPIENT", DIM(), scale::BODY);
    let mut to = [0u8; 42];
    to[0] = b'0';
    to[1] = b'x';
    to[2..2 + state.send_to_len].copy_from_slice(&state.send_to_hex[..state.send_to_len]);
    let to_str = if state.send_to_len == 0 {
        "Paste a 0x recipient address"
    } else {
        core::str::from_utf8(&to[..2 + state.send_to_len]).unwrap_or("")
    };
    field(fb, ix, field_to, iw, to_str, state.send_focus == SEND_FIELD_TO);
    let ok = state.send_to_len == 40;
    let _ = fb.text_ttf(
        ix as i32,
        hint_to as i32,
        if ok { "20-byte address" } else { "enter 40 hex characters" },
        if ok { ACCENT() } else { MUTED() },
        scale::BODY,
    );

    // The amount exactly as entered, in the selected asset.
    let amount_label = if state.send_token == 1 { "AMOUNT (NOX)" } else { "AMOUNT (ETH)" };
    let _ = fb.text_ttf(ix as i32, label_amount as i32, amount_label, DIM(), scale::BODY);
    let mut ab = [0u8; 24];
    let an = super::format_typed::format_typed(&state.send_amount, &mut ab);
    field(
        fb,
        ix,
        field_amount,
        iw,
        core::str::from_utf8(&ab[..an]).unwrap_or("0"),
        state.send_focus == SEND_FIELD_AMOUNT,
    );

    // The live network fee, not a fabricated dollar figure.
    let _ = fb.text_ttf(ix as i32, label_fee as i32, "NETWORK FEE", DIM(), scale::BODY);
    ui::bordered(fb, ix, box_fee, iw, FEE_H, PANEL_2(), LINE2());
    let mut gb = [0u8; 32];
    let gn = gwei(state.fee_wei, &mut gb);
    let fee_txt = if state.fee_ready {
        core::str::from_utf8(&gb[..gn]).unwrap_or("\u{2014}")
    } else {
        "fetching\u{2026}"
    };
    let fee_text = (box_fee + space(3)) as i32;
    let _ = fb.text_ttf((ix + 14) as i32, fee_text, "Gas price", MUTED(), scale::BODY);
    let fw = fb.measure_ttf(fee_txt, scale::BODY).max(0) as u32;
    let _ = fb.text_ttf((ix + iw - 14 - fw) as i32, fee_text, fee_txt, FG(), scale::BODY);

    ui::bordered(fb, ix, box_warn, iw, WARN_H, 0xFF17_130A, 0xFF5A_4A1E);
    fb.fill_rect(ix + 14, box_warn + space(4), 10, 10, AMBER());
    let _ = fb.text_ttf(
        (ix + 34) as i32,
        (box_warn + space(3)) as i32,
        "Verify the recipient. Transfers cannot be reversed.",
        AMBER(),
        scale::BODY,
    );

    ui::primary(fb, ix, button, 150, b"Sign & send");
    super::paint_send_side::paint_send_side(state, fb);
}

// wei-per-gas to a "N.NN gwei" string, two decimals so a sub-gwei price shows.
fn gwei(wei: u64, out: &mut [u8]) -> usize {
    let whole = wei / 1_000_000_000;
    let cents = (wei % 1_000_000_000) / 10_000_000;
    let mut gb = [0u8; 20];
    let n = super::format_u64::format_u64(whole, &mut gb);
    out[..n].copy_from_slice(&gb[..n]);
    out[n] = b'.';
    out[n + 1] = b'0' + ((cents / 10) % 10) as u8;
    out[n + 2] = b'0' + (cents % 10) as u8;
    out[n + 3..n + 8].copy_from_slice(b" gwei");
    n + 8
}

fn asset_tab(fb: &mut PaintBuffer, x: u32, y: u32, label: &str, on: bool) {
    if on {
        fb.fill_rect(x, y, 74, 26, ACCENT());
    } else {
        ui::edge(fb, x, y, 74, 26, LINE2());
    }
    let c = if on { INK() } else { MUTED() };
    let tw = fb.measure_ttf(label, scale::BODY).max(0) as u32;
    let _ = fb.text_ttf((x + 37 - tw / 2) as i32, (y + 6) as i32, label, c, scale::BODY);
}

fn field(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, val: &str, active: bool) {
    let e = if active { ACCENT() } else { LINE2() };
    ui::bordered(fb, x, y, w, 40, PANEL_2(), e);
    let _ = fb.text_ttf((x + 12) as i32, (y + 11) as i32, val, FG(), scale::BODY);
}
