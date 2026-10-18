extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::{
    mk_mmap, mk_munmap, mk_surface_register, mk_surface_share, SurfaceDescriptor,
    SURFACE_FORMAT_ARGB8888,
};

/// The input as a surface only `codec` maps, read-only (K20).
pub(super) fn share_input(
    bytes: &[u8],
    codec: u32,
) -> Result<(u64, u64, *mut u8, usize), &'static str> {
    let byte_len = bytes.len() as u64;
    let padded = ((bytes.len() + 3) & !3).max(4);
    let base = mk_mmap(core::ptr::null_mut(), padded, 0x3, 0x22, -1, 0);
    if (base as isize) <= 0 {
        return Err("input mmap failed");
    }
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), base, bytes.len());
        if padded > bytes.len() {
            core::ptr::write_bytes(base.add(bytes.len()), 0, padded - bytes.len());
        }
    }
    let width = (padded / 4) as u32;
    let desc = SurfaceDescriptor {
        width,
        height: 1,
        stride: width * 4,
        format: SURFACE_FORMAT_ARGB8888,
        byte_len: padded as u64,
        base_va: base as u64,
        flags: 0,
        owner_pid: 0,
        writable: 0,
    };
    let sid = mk_surface_register(&desc);
    if sid < 0 {
        let _ = mk_munmap(base, padded);
        return Err("input register failed");
    }
    let handle = mk_surface_share(sid as u64, codec, 0);
    if handle <= 0 {
        let _ = mk_munmap(base, padded);
        return Err("input share failed");
    }
    Ok((handle as u64, byte_len, base, padded))
}

pub(super) fn copy_surface(base_va: usize, stride_bytes: u32, w: u32, h: u32) -> Vec<u32> {
    let mut out = vec![0u32; w as usize * h as usize];
    let mut y = 0usize;
    while y < h as usize {
        let row = (base_va + y * stride_bytes as usize) as *const u32;
        let mut x = 0usize;
        while x < w as usize {
            out[y * w as usize + x] = unsafe { core::ptr::read_volatile(row.add(x)) };
            x += 1;
        }
        y += 1;
    }
    out
}
