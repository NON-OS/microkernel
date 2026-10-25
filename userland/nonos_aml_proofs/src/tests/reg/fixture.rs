// NONOS Operating System (AGPL-3.0-or-later)
//! A case as tables: the DSDT and each later table "OEMk", with the DSDT
//! holding each later table's bytes for Load and a method per load path.

use super::gen::{reg_body, Case};
use crate::enc::*;

fn path(tag: u64) -> String {
    format!("\\D{tag:02}")
}

fn body(c: &Case, t: usize) -> Vec<u8> {
    let mut out = Vec::new();
    for d in &c.devs {
        let reg = method("_REG", 2, &reg_body(d.tag));
        let here = d.reg_in == Some(t);
        if d.table == t {
            let mut inner: Vec<u8> = Vec::new();
            for (i, s) in d.spaces.iter().enumerate() {
                inner.extend(region(&format!("R{i}"), *s, 0x10 * i as u64, 1));
            }
            if here {
                inner.extend(&reg);
            }
            out.extend(device(&path(d.tag), &inner));
        } else if here {
            out.extend(scope(&path(d.tag), &reg));
        }
    }
    out
}

pub fn sig(k: usize) -> [u8; 4] {
    [b'O', b'E', b'M', b'0' + k as u8]
}

/// The DSDT's AML, and the later tables whole.
pub fn tables(c: &Case) -> (Vec<u8>, Vec<Vec<u8>>) {
    let later: Vec<Vec<u8>> = (1..=c.ssdts).map(|k| table(&sig(k), 2, &body(c, k))).collect();
    let mut dsdt = body(c, 0);
    for (i, t) in later.iter().enumerate() {
        let k = i + 1;
        let text = String::from_utf8(sig(k).to_vec()).unwrap();
        let lt = e(
            0x5B1F,
            &[&string(&text), &string(""), &string(""), &string("\\"), &string(""), &int(0)],
        );
        let ld = e(0x5B20, &[&name(&format!("\\TB{k}")), &local(0)]);
        dsdt.extend(name_obj(&format!("\\TB{k}"), &buffer(t.len() as u64, t)));
        dsdt.extend(method(&format!("\\LT{k}"), 0, &ret(&lt)));
        dsdt.extend(method(&format!("\\LB{k}"), 0, &cat(&[&ld, &ret(&local(0))])));
    }
    (dsdt, later)
}
