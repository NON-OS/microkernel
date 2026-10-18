// NONOS Operating System (AGPL-3.0-or-later)
//! Frames: the opcode and 24-bit address first, then an output report's
//! type, length and id; a read clocks in as many bytes as it asks for.

use nonos_spi_hid::{read_frame, write_frame, Params, OUT_REPORT_DESCRIPTOR};

const P: Params =
    Params { read_opcode: 0x0B, write_opcode: 0x02, input_address: 0x1000, output_address: 0x2000 };

#[test]
fn a_write_is_its_head_then_the_report() {
    let f = write_frame(&P, OUT_REPORT_DESCRIPTOR, 0, &[]).unwrap();
    assert_eq!(f, [0x02, 0x00, 0x20, 0x00, OUT_REPORT_DESCRIPTOR, 0, 0, 0]);
    let f = write_frame(&P, 0x05, 3, &[9, 8]).unwrap();
    assert_eq!(f[4..], [0x05, 2, 0, 3, 9, 8]);
    assert!(write_frame(&P, 0x05, 3, &vec![0; 70_000]).is_none(), "longer than a length says");
}

#[test]
fn a_read_asks_for_exactly_its_length_after_the_head() {
    let f = read_frame(&P, 4);
    assert_eq!(f, [0x0B, 0x00, 0x10, 0x00, 0, 0, 0, 0]);
    assert_eq!(read_frame(&P, 100).len(), 104);
    let high = Params { input_address: 0xAB_CDEF, ..P };
    assert_eq!(read_frame(&high, 0)[1..4], [0xAB, 0xCD, 0xEF], "big-endian address");
}
