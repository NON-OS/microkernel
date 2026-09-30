/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * One delivery from the input router as the event the window's key router
 * takes. A line feed is Enter, as setup reads it.
 */

use nonos_app_skeleton::wire::NINP_MAGIC;
use nonos_app_skeleton::{InputEvent, KEY_ENTER};

const HDR_LEN: usize = 8;
pub(super) const DELIVERY_LEN: usize = HDR_LEN + 32;
const KEY_LINE_FEED: u32 = 0x0A;

pub(super) fn parse(buf: &[u8]) -> Option<InputEvent> {
    if buf.len() < DELIVERY_LEN || buf[0..4] != NINP_MAGIC.to_le_bytes() {
        return None;
    }
    let mut e = InputEvent::from_delivery(&buf[HDR_LEN..])?;
    if e.code == KEY_LINE_FEED {
        e.code = KEY_ENTER;
    }
    Some(e)
}
