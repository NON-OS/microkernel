// NONOS Operating System (AGPL-3.0-or-later)
//! The named objects and statements the fixtures use most, in ASL's terms.

use super::{cat, name, op, pkg};

pub fn scope(path: &str, body: &[u8]) -> Vec<u8> {
    pkg(0x10, &cat(&[&name(path), body]))
}

pub fn device(path: &str, body: &[u8]) -> Vec<u8> {
    pkg(0x5B82, &cat(&[&name(path), body]))
}

/// Method(path, args) { body }, not serialized, sync level 0.
pub fn method(path: &str, args: u8, body: &[u8]) -> Vec<u8> {
    pkg(0x14, &cat(&[&name(path), &[args & 7], body]))
}

pub fn name_obj(path: &str, value: &[u8]) -> Vec<u8> {
    cat(&[&op(0x08), &name(path), value])
}

pub fn if_(pred: &[u8], body: &[u8]) -> Vec<u8> {
    pkg(0xA0, &cat(&[pred, body]))
}

pub fn else_(body: &[u8]) -> Vec<u8> {
    pkg(0xA1, body)
}

/// External(path, MethodObj, args).
pub fn external_method(path: &str, args: u8) -> Vec<u8> {
    cat(&[&op(0x15), &name(path), &[8, args]])
}

pub fn thermal_zone(path: &str, body: &[u8]) -> Vec<u8> {
    pkg(0x5B85, &cat(&[&name(path), body]))
}

pub fn power_res(path: &str, level: u8, order: u16, body: &[u8]) -> Vec<u8> {
    pkg(0x5B84, &cat(&[&name(path), &[level], &order.to_le_bytes(), body]))
}

/// Processor(path, ProcID, PblkAddr, PblkLen) { body }.
pub fn processor(path: &str, id: u8, pblk: u32, len: u8, body: &[u8]) -> Vec<u8> {
    pkg(0x5B83, &cat(&[&name(path), &[id], &pblk.to_le_bytes(), &[len], body]))
}
