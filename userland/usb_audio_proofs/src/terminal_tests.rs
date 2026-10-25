// NONOS Operating System (AGPL-3.0-or-later)
//! What a device plays into, from its Output Terminals.

use crate::devices::*;
use crate::uac::{outputs, Outputs};

fn with_terminals(kinds: &[u16]) -> Vec<u8> {
    let ac = iface(0, 0, 1, 0);
    let terms: Vec<Vec<u8>> = kinds.iter().map(|&k| output_terminal(k)).collect();
    let mut parts: Vec<&[u8]> = vec![&ac];
    parts.extend(terms.iter().map(|t| t.as_slice()));
    config(&parts)
}

#[test]
fn a_speaker_a_headset_and_an_unknown_kind() {
    let speaker = Outputs { speaker: true, ..Outputs::default() };
    let worn = Outputs { headphone: true, ..Outputs::default() };
    let line = Outputs { line: true, ..Outputs::default() };
    assert_eq!(outputs(&qemu_usb_audio()), speaker);
    assert_eq!(outputs(&headset()), worn);
    assert_eq!(outputs(&with_terminals(&[0x0302])), worn);
    assert_eq!(outputs(&with_terminals(&[0x0305])), speaker);
    assert_eq!(outputs(&with_terminals(&[0x0603])), line);
    let both = Outputs { speaker: true, headphone: true, line: false };
    assert_eq!(outputs(&with_terminals(&[0x0301, 0x0302])), both);
}

#[test]
fn a_capture_terminal_or_none_counts_as_line_out() {
    let line = Outputs { line: true, ..Outputs::default() };
    assert_eq!(outputs(&with_terminals(&[0x0101])), line);
    assert_eq!(outputs(&with_terminals(&[])), line);
}

#[test]
fn a_streaming_interface_descriptor_is_not_a_terminal() {
    // Subtype 3 inside an AudioStreaming interface is FORMAT_SPECIFIC.
    let (ac, as_if) = (iface(0, 0, 1, 0), iface(1, 1, 2, 0));
    let c = config(&[&ac, &output_terminal(0x0301), &as_if, &output_terminal(0x0302)]);
    assert_eq!(outputs(&c), Outputs { speaker: true, ..Outputs::default() });
}
