use nonos_libc::mk_ipc_call;
use nonos_policy_proto::{Header, HDR_LEN, IPC_PAYLOAD_MAX, KIND_BOOL, KIND_I8, KIND_STR, KIND_U8};
use nonos_policy_proto::{E_BAD_LEN, OP_SET, STR_MAX};

fn finish(port: u32, tx: &[u8]) -> Result<(), i32> {
    let mut rx = [0u8; IPC_PAYLOAD_MAX];
    let n = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if n < HDR_LEN as i64 {
        return Err(-11);
    }
    let reply = Header::decode(&rx[..HDR_LEN]).ok_or(-11)?;
    if reply.status != 0 {
        return Err(reply.status as i32);
    }
    Ok(())
}

pub fn set_bool(port: u32, field: u32, value: bool) -> Result<(), i32> {
    let mut tx = [0u8; HDR_LEN + 1];
    let hdr = Header { op: OP_SET, field, kind: KIND_BOOL, status: 0, payload_len: 1 };
    hdr.encode(&mut tx[..HDR_LEN]);
    tx[HDR_LEN] = if value { 1 } else { 0 };
    finish(port, &tx)
}

pub fn set_u8(port: u32, field: u32, value: u8) -> Result<(), i32> {
    let mut tx = [0u8; HDR_LEN + 1];
    let hdr = Header { op: OP_SET, field, kind: KIND_U8, status: 0, payload_len: 1 };
    hdr.encode(&mut tx[..HDR_LEN]);
    tx[HDR_LEN] = value;
    finish(port, &tx)
}

pub fn set_i8(port: u32, field: u32, value: i8) -> Result<(), i32> {
    let mut tx = [0u8; HDR_LEN + 1];
    let hdr = Header { op: OP_SET, field, kind: KIND_I8, status: 0, payload_len: 1 };
    hdr.encode(&mut tx[..HDR_LEN]);
    tx[HDR_LEN] = value as u8;
    finish(port, &tx)
}

/* A value longer than the wire takes is refused here, never cut short. */
pub fn set_str(port: u32, field: u32, value: &[u8]) -> Result<(), i32> {
    if value.len() > STR_MAX {
        return Err(E_BAD_LEN as i32);
    }
    let mut tx = [0u8; HDR_LEN + STR_MAX];
    let len = value.len();
    let hdr = Header { op: OP_SET, field, kind: KIND_STR, status: 0, payload_len: len as u16 };
    hdr.encode(&mut tx[..HDR_LEN]);
    tx[HDR_LEN..HDR_LEN + len].copy_from_slice(value);
    finish(port, &tx[..HDR_LEN + len])
}
