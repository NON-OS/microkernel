// NONOS Operating System (AGPL-3.0-or-later)
//! Every set bit of a mask is printed once: by the kernel's name when this
//! build knows it, else as `unknown bit N`. None is dropped.

use super::model::kernel_bits;
use crate::command::builtin::audit::join::join;
use crate::command::builtin::audit::names::names;
use crate::command::builtin::audit::table::line;

#[test]
fn no_bit_is_dropped_and_the_unknown_are_said() {
    let k = kernel_bits();
    let by_bit: std::collections::BTreeMap<u64, &String> = k.iter().map(|(n, b)| (*b, n)).collect();
    let (mut x, mut unknown) = (5u64, 0);
    for i in 0..20_000u64 {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let m = if i < 64 { 1u64 << i } else { x };
        let got = names(m);
        assert_eq!(got.len(), m.count_ones() as usize, "{m:#x}: one name per bit");
        let want: Vec<String> = (0..64)
            .filter(|b| m & (1u64 << b) != 0)
            .map(|b| match by_bit.get(&(1u64 << b)) {
                Some(n) => n.to_string(),
                None => format!("unknown bit {b}"),
            })
            .collect();
        let got: Vec<String> = got.iter().map(|n| String::from_utf8(n.clone()).unwrap()).collect();
        assert_eq!(got, want, "{m:#x}");
        unknown += got.iter().filter(|n| n.starts_with("unknown")).count();
    }
    let row = join(&super::model::entry(3, 1 << 63 | 1 << 2, 0), &[(3, b"x", false)]).unwrap();
    assert!(String::from_utf8(line(&row[0])).unwrap().ends_with("Network, unknown bit 63"));
    println!("{unknown} unknown bits named");
    assert!(unknown >= 200_000, "{unknown}");
}
