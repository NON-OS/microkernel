// NONOS Operating System (AGPL-3.0-or-later)
//! The shared function-0 rule (`platform::dsm_mask`) on seeded answers of
//! every shape against `dsm_model::mask_of`, and `offers` at its edges.

use super::dsm_gen::{below, Form};
use super::dsm_model::mask_of;
use crate::object::Object;
use crate::platform::{offered_mask, offers};

fn object(f: &Form) -> Option<Object> {
    match f {
        Form::Absent => None,
        Form::Buf(b) => Some(Object::buffer(b.clone())),
        Form::Int(n) => Some(Object::Integer(*n)),
        Form::Str => Some(Object::string(b"1")),
        Form::Pkg => Some(Object::package(vec![Object::Integer(1)])),
    }
}

#[test]
fn every_shape_reads_as_the_model_says() {
    let s = &mut 0x1234_5678_9ABC_DEF1u64;
    let mut offered = 0;
    for _ in 0..20_000 {
        let len = below(s, 13) as usize;
        let form = match below(s, 5) {
            0 => Form::Int(below(s, u64::MAX) | below(s, 2)),
            1 => Form::Str,
            2 => Form::Pkg,
            3 => Form::Absent,
            _ => Form::Buf((0..len).map(|_| below(s, 256) as u8).collect()),
        };
        let got = offered_mask(object(&form).as_ref());
        assert_eq!(got, mask_of(&form), "{form:?}");
        offered += usize::from(got != 0);
    }
    assert!(offered >= 4_000, "{offered}");
}

#[test]
fn offers_needs_bit_zero_and_stops_at_sixty_three() {
    assert!(offers(0b10_0001, 5));
    assert!(!offers(0b10_0000, 5), "bit 0 clear offers nothing");
    assert!(!offers(0b00_0001, 5));
    assert!(offers(u64::MAX, 63));
    assert!(!offers(u64::MAX, 64) && !offers(u64::MAX, u64::MAX), "no function past 63");
    let nine = offered_mask(Some(&Object::buffer(vec![0x01, 0x02, 0, 0, 0, 0, 0, 0, 0xFF])));
    assert_eq!(nine, 0x0201, "little-endian, and the ninth byte unread");
}
