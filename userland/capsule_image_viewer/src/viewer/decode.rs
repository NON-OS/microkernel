// NONOS Operating System (AGPL-3.0-or-later)
extern crate alloc;
use crate::viewer::budget::refuse_pixels;
use alloc::vec::Vec;
use nonos_app_skeleton::discover::lookup_service;
use nonos_app_skeleton::wire::build_request;
use nonos_libc::{
    mk_ipc_call, mk_munmap, mk_surface_attach, mk_surface_release, SurfaceDescriptor,
};

use super::decode_op::op_for;
use super::decode_surface::{copy_surface, share_input};

const NIMG: u32 = 0x474D_494E;

pub struct Decoded {
    pub px: Vec<u32>,
    pub w: u32,
    pub h: u32,
}

pub fn decode(bytes: &[u8], name: &[u8]) -> Result<Decoded, &'static str> {
    let op = op_for(name)?;
    let svc = lookup_service(b"image_codec").ok_or("no codec")?;
    let (in_handle, in_len, in_base, in_pad) = share_input(bytes, svc.pid)?;
    let mut payload = [0u8; 16];
    payload[0..8].copy_from_slice(&in_handle.to_le_bytes());
    payload[8..16].copy_from_slice(&in_len.to_le_bytes());
    let tx = build_request(NIMG, op, 1, &payload);
    let mut rx = [0u8; 56];
    let rc = mk_ipc_call(svc.port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    let _ = mk_surface_release(in_handle);
    let _ = mk_munmap(in_base, in_pad);
    if rc < 48 {
        return Err("codec call failed");
    }
    if i32::from_le_bytes([rx[20], rx[21], rx[22], rx[23]]) != 0 {
        return Err("decode error");
    }
    let handle =
        u64::from_le_bytes([rx[24], rx[25], rx[26], rx[27], rx[28], rx[29], rx[30], rx[31]]);
    let w = u32::from_le_bytes([rx[32], rx[33], rx[34], rx[35]]);
    let h = u32::from_le_bytes([rx[36], rx[37], rx[38], rx[39]]);
    let mut desc = SurfaceDescriptor::default();
    let va = mk_surface_attach(handle, &mut desc as *mut _);
    if va <= 0 {
        // The codec handed this handle over; it is ours to give back whether
        // or not it could be mapped, or the decoder's surface stays held.
        let _ = mk_surface_release(handle);
        return Err("attach failed");
    }
    if w != desc.width || h != desc.height {
        let _ = mk_surface_release(handle);
        return Err("codec dim mismatch");
    }
    if (desc.width as u64) * (desc.height as u64) * 4 > desc.byte_len {
        let _ = mk_surface_release(handle);
        return Err("surface too small");
    }
    if (desc.stride as u64) < (desc.width as u64) * 4
        || (desc.stride as u64) * (desc.height as u64) > desc.byte_len
    {
        let _ = mk_surface_release(handle);
        return Err("bad surface geometry");
    }
    // The decoder's surface is its memory; the copy below is the heap's.
    if let Some(why) = refuse_pixels(desc.width, desc.height) {
        let _ = mk_surface_release(handle);
        return Err(why);
    }
    let px = copy_surface(va as usize, desc.stride, desc.width, desc.height);
    let _ = mk_surface_release(handle);
    Ok(Decoded { px, w: desc.width, h: desc.height })
}
