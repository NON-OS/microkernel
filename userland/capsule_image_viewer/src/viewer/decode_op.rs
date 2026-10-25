const OP_PNG: u16 = 0x0002;
const OP_BMP: u16 = 0x0003;
const OP_JPEG: u16 = 0x0005;
const OP_GIF: u16 = 0x0006;

pub(super) fn op_for(name: &[u8]) -> Result<u16, &'static str> {
    let dot = name.iter().rposition(|&b| b == b'.').ok_or("no ext")? + 1;
    let e = &name[dot..];
    let low = |x: &[u8], y: &[u8]| {
        x.len() == y.len() && x.iter().zip(y).all(|(a, b)| a.to_ascii_lowercase() == *b)
    };
    if low(e, b"png") {
        Ok(OP_PNG)
    } else if low(e, b"bmp") {
        Ok(OP_BMP)
    } else if low(e, b"jpg") || low(e, b"jpeg") {
        Ok(OP_JPEG)
    } else if low(e, b"gif") {
        Ok(OP_GIF)
    } else {
        Err("unsupported ext")
    }
}
