// NONOS Operating System (AGPL-3.0-or-later)
//! Small trees for the console finder's proofs.

use crate::arch::fdt::find::uart::{find, UartInfo};
use crate::arch::fdt::Fdt;
use crate::builder::{cells, text, Builder, END};

pub type Props = &'static [(&'static str, &'static [u32])];

/// A root with a 50 MHz fixed clock (phandle 1), a clock controller
/// (phandle 2), and one serial node per entry, in order.
fn tree(nodes: &[(&str, Props)], stdout: Option<&str>) -> Vec<u8> {
    let mut b = Builder::default().begin("");
    b = b.prop("#address-cells", &cells(&[1])).prop("#size-cells", &cells(&[1]));
    b = b.begin("osc").prop("compatible", &text("fixed-clock"));
    b = b.prop("clock-frequency", &cells(&[50_000_000])).prop("phandle", &cells(&[1])).end();
    b = b.begin("cprman").prop("clock-frequency", &cells(&[9])).prop("phandle", &cells(&[2])).end();
    for (i, (compat, props)) in nodes.iter().enumerate() {
        b = b.begin(&format!("serial@{i}")).prop("compatible", &text(compat));
        b = b.prop("reg", &cells(&[0x1000 * (i as u32 + 1), 0x100]));
        for (name, value) in props.iter() {
            b = b.prop(name, &cells(value));
        }
        b = b.end();
    }
    if let Some(path) = stdout {
        b = b.begin("chosen").prop("stdout-path", &text(path)).end();
    }
    b.end().token(END).finish()
}

pub fn console(nodes: &[(&str, Props)], stdout: Option<&str>) -> Option<UartInfo> {
    find(&Fdt::from_bytes(&tree(nodes, stdout)).unwrap()).unwrap()
}
