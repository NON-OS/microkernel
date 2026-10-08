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

use alloc::vec::Vec;

use nonos_libc::{mk_ipc_call, mk_ipc_call_timeout};

const NOTK_MAGIC: u32 = 0x4E4F_544B;
const HDR_LEN: usize = 16;
const TOOLKIT_OP_COMPONENT_RENDER: u16 = 0x0003;
const STATUS_OK: u16 = 0;
/// `ComponentKind::Frame`: a whole-window handshake the toolkit answers with the
/// theme revision, attaching and painting nothing.
const COMPONENT_KIND_FRAME: u16 = 3;
/// A frame answer is the theme revision as one u32.
const FRAME_REPLY_LEN: usize = 4;
/// The toolkit answers a frame without mapping a surface, so the budget is a
/// frame's scale: a slow or missing toolkit never stalls the app's paint loop.
const FRAME_BUDGET_MS: u64 = 8;

pub fn component_render(port: u32, request_id: u32, payload: &[u8]) -> Result<(), &'static str> {
    let mut request = Vec::with_capacity(HDR_LEN + payload.len());
    request.extend_from_slice(&NOTK_MAGIC.to_le_bytes());
    request.extend_from_slice(&TOOLKIT_OP_COMPONENT_RENDER.to_le_bytes());
    request.extend_from_slice(&0u16.to_le_bytes());
    request.extend_from_slice(&request_id.to_le_bytes());
    request.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    request.extend_from_slice(payload);

    let mut reply = [0u8; HDR_LEN];
    let rc =
        mk_ipc_call(port as u64, request.as_ptr(), request.len(), reply.as_mut_ptr(), reply.len());
    if rc < HDR_LEN as i64 {
        return Err("toolkit component_render short reply");
    }
    let status = u16::from_le_bytes([reply[6], reply[7]]);
    if status != STATUS_OK {
        return Err("toolkit component_render rejected");
    }
    Ok(())
}

/// Tell the toolkit of a whole frame the app drew, and read back the live theme
/// revision, so the app can fetch the theme only when it changed. Attaches and
/// paints nothing; the toolkit holds no GraphicsSurfaceMap.
pub fn ui_frame(
    port: u32,
    request_id: u32,
    surface_handle: u64,
    width: u32,
    height: u32,
) -> Result<u32, &'static str> {
    let mut payload = [0u8; 28];
    payload[0..8].copy_from_slice(&surface_handle.to_le_bytes());
    payload[16..20].copy_from_slice(&width.to_le_bytes());
    payload[20..24].copy_from_slice(&height.to_le_bytes());
    payload[24..26].copy_from_slice(&COMPONENT_KIND_FRAME.to_le_bytes());

    let mut request = Vec::with_capacity(HDR_LEN + payload.len());
    request.extend_from_slice(&NOTK_MAGIC.to_le_bytes());
    request.extend_from_slice(&TOOLKIT_OP_COMPONENT_RENDER.to_le_bytes());
    request.extend_from_slice(&0u16.to_le_bytes());
    request.extend_from_slice(&request_id.to_le_bytes());
    request.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    request.extend_from_slice(&payload);

    let mut reply = [0u8; HDR_LEN + FRAME_REPLY_LEN];
    let rc = mk_ipc_call_timeout(
        port as u64,
        request.as_ptr(),
        request.len(),
        reply.as_mut_ptr(),
        reply.len(),
        FRAME_BUDGET_MS,
    );
    if rc < (HDR_LEN + FRAME_REPLY_LEN) as i64 {
        return Err("toolkit ui_frame short reply");
    }
    let status = u16::from_le_bytes([reply[6], reply[7]]);
    if status != STATUS_OK {
        return Err("toolkit ui_frame rejected");
    }
    Ok(u32::from_le_bytes([
        reply[HDR_LEN],
        reply[HDR_LEN + 1],
        reply[HDR_LEN + 2],
        reply[HDR_LEN + 3],
    ]))
}
