use crate::hid::{
    input_len, input_register, parse_mouse_layout, parse_report_descriptor, probe_addr, probe_bus,
    HID_DESC_LEN,
};
use crate::i2c_client::{query_acpi_hid, resolve, write_read};
use crate::state::State;

/// Upper bound on the report descriptor we will read and parse. Real precision
/// touchpads (Synaptics, Elan) ship descriptors of several hundred bytes and up
/// to roughly a kilobyte, so a smaller cap silently drops the absolute-touch
/// layout and strands the device on the relative boot-mouse path.
const REPORT_DESC_MAX: usize = 1024;

pub fn run() -> Result<State, &'static str> {
    let (port, pid) = resolve().ok_or("i2c-hid: missing i2c controller")?;
    let mut state = State::new(port, pid);
    reprobe(&mut state);
    Ok(state)
}

pub fn reprobe(state: &mut State) {
    state.probes += 1;
    // Prefer the address the kernel recovered from ACPI so the driver binds to
    // the exact device; fall back to probing the common addresses only when the
    // firmware declared none.
    let acpi = query_acpi_hid(state.i2c_port).and_then(|(addr, reg)| {
        probe_addr(state.i2c_port, addr, reg, &mut state.descriptor).map(|_| (addr, HID_DESC_LEN))
    });
    if let Some((addr, len)) = acpi.or_else(|| probe_bus(state.i2c_port, &mut state.descriptor)) {
        state.addr = addr;
        state.input_register = input_register(&state.descriptor);
        state.input_len = input_len(&state.descriptor);
        // Wake the device before touching the report descriptor: a real
        // touchpad answers nothing until SET_POWER(ON) + RESET.
        state.woke =
            crate::hid::wake(state.i2c_port, state.addr, &state.descriptor, state.input_register);
        let Some(report_desc) = read_report_descriptor(state) else {
            // Bound only once the report descriptor reads: without it the
            // reports cannot be decoded, and a later reprobe gets another try.
            say_once(state, "driver.i2c_hid: the touchpad answered but its report descriptor could not be read; retrying\n");
            return;
        };
        state.descriptor_len = len;
        state.touch_layout = parse_report_descriptor(&report_desc);
        state.mouse_layout = parse_mouse_layout(&report_desc);
        // Drive the pad's reporting configuration: input mode = touchpad (the
        // only collection PTP-class pads generate reports on; the vestigial
        // mouse collection is a stub Windows and Linux never use), surface
        // and button switches on. Read-modify-write per feature report,
        // writing only when a bit differs, so a correctly-configured pad
        // sees no writes, while one left muted or mode-switched by a
        // previous session gets repaired. Read pacing comes from the GPIO
        // doorbell. Best-effort: pads without the features are unaffected.
        let _ = crate::hid::configure_reporting(
            state.i2c_port,
            state.addr,
            &state.descriptor,
            &state.touch_layout,
        );
        dump_layout(state);
    } else if state.probes == UNANSWERED_PROBES {
        say_once(state, "driver.i2c_hid: no HID-over-I2C touchpad answers on the I2C bus; if this machine has one, it is on a controller or address NONOS cannot reach\n");
    }
}

/// Reprobes (about 0.5 s apart) after which a touchpad that never answered
/// is reported on the console, once.
const UNANSWERED_PROBES: u64 = 20;

fn say_once(state: &mut State, msg: &str) {
    if !state.said {
        state.said = true;
        crate::diag::line(alloc::string::String::from(msg));
    }
}

// One-shot bind report on the boot console: the parsed field map in bit
// offsets, so a photograph of the screen pins exactly how this driver is
// reading the device's frames.
fn dump_layout(state: &State) {
    let l = &state.touch_layout;
    crate::diag::line(alloc::format!(
        "[i2chid] bind addr={:#x} rid={} maxin={} inreg={:#x} woke={}\n",
        state.addr,
        l.report_id,
        state.input_len,
        state.input_register,
        state.woke,
    ));
    crate::diag::line(alloc::format!(
        "[i2chid] x@{}+{} max={} y@{}+{} max={} tip@{}+{} cnt@{}+{} btn@{}+{} conf@{}+{}\n",
        l.x.bit_offset,
        l.x.bit_size,
        l.x.logical_max,
        l.y.bit_offset,
        l.y.bit_size,
        l.y.logical_max,
        l.tip.bit_offset,
        l.tip.bit_size,
        l.contact_count.bit_offset,
        l.contact_count.bit_size,
        l.button.bit_offset,
        l.button.bit_size,
        l.confidence.bit_offset,
        l.confidence.bit_size,
    ));
}

// Fetch the HID report descriptor named in the HID descriptor and parse it into
// a touchpad field map. A device that is not an absolute touchpad simply yields
// an empty layout, and the driver stays on the relative decode path.
const REPORT_DESC_ATTEMPTS: u32 = 3;
const REPORT_DESC_RETRY_MS: u64 = 20;

/// The report descriptor, read whole. A device just out of RESET can still
/// be busy, so a failed or short read is retried a few times.
fn read_report_descriptor(state: &State) -> Option<alloc::vec::Vec<u8>> {
    let desc = &state.descriptor;
    let rd_len = u16::from_le_bytes([desc[4], desc[5]]) as usize;
    let rd_reg = u16::from_le_bytes([desc[6], desc[7]]);
    if !(4..=REPORT_DESC_MAX).contains(&rd_len) {
        return None;
    }
    let mut buf = alloc::vec![0u8; rd_len];
    for attempt in 0..REPORT_DESC_ATTEMPTS {
        if attempt > 0 {
            let _ = nonos_libc::mk_idle_ms(REPORT_DESC_RETRY_MS);
        }
        if write_read(state.i2c_port, state.addr, &rd_reg.to_le_bytes(), &mut buf) == Some(rd_len) {
            return Some(buf);
        }
    }
    None
}
