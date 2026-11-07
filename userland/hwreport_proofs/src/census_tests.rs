// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The kernel's boot census rule (src/hardware/boot_census/rule.rs, mounted
//! by path) for a laptop, a desktop and a VM, each line as the kernel writes
//! it, read back by the report's `census` parser and carried in a report
//! that `parse::check` accepts.

#[path = "../../../src/hardware/boot_census/rule.rs"]
mod rule;
#[path = "../../../src/hardware/boot_census/when.rs"]
mod when;

use nonos_hwreport::census::{parse, Census, State};
use nonos_hwreport::parse::check;
use nonos_hwreport::report::Report;
use rule::{rows, Battery, Count, Declared, Facts, Row, Seen, Tally};

fn one(up: u8) -> Count {
    Count { expected: 1, up, ..Count::default() }
}

/// a reference laptop: a battery, AC, lid, two thermal zones; PS/2 keyboard,
/// I2C touchpad, Xe-LP, NVMe, Intel Wi-Fi, xHCI, HDA; all up.
fn laptop() -> Facts {
    Facts {
        acpi: Some(Declared { battery: true, ac: true, lid: true, thermal_zones: 2 }),
        acpi_capsule: true,
        battery: Battery::Charged,
        ac_known: true,
        lid_known: true,
        boot_framebuffer: true,
        keyboard: one(1),
        pointer: one(1),
        display: one(1),
        storage: one(1),
        network: one(1),
        usb: one(1),
        audio: one(1),
    }
}

fn desktop() -> Facts {
    Facts {
        acpi: Some(Declared { thermal_zones: 1, ..Declared::default() }),
        battery: Battery::Empty,
        pointer: Count::default(),
        ..laptop()
    }
}

/// QEMU q35 with OVMF: no battery, AC, lid or thermal zone; PS/2, the
/// firmware framebuffer, virtio-blk, e1000, xHCI, no audio.
fn vm() -> Facts {
    Facts {
        acpi: Some(Declared::default()),
        battery: Battery::Empty,
        ac_known: false,
        lid_known: false,
        pointer: Count::default(),
        display: Count::default(),
        audio: Count::default(),
        ..laptop()
    }
}

fn seen(f: &Facts, name: &str) -> Seen {
    rows(f).iter().find(|r| r.class.name() == name).map(|r| r.seen).expect("a row")
}

fn line(r: &Row) -> Vec<u8> {
    let mut v = Vec::new();
    rule::render(r, &mut |b| v.extend_from_slice(b));
    v
}

fn summary(f: &Facts) -> Vec<u8> {
    let mut v = Vec::new();
    rule::render_summary(&Tally::of(&rows(f)), &mut |b| v.extend_from_slice(b));
    v
}

#[test]
fn the_laptop_is_green_with_every_row_up_but_thermal_held() {
    let f = laptop();
    let t = Tally::of(&rows(&f));
    assert_eq!((t.up, t.absent, t.held, t.missing), (10, 0, 1, 0));
    assert_eq!(summary(&f), b"[CENSUS] summary up=10 absent=0 held=1 missing=0 verdict=green");
    assert_eq!(
        line(&rows(&f)[0]),
        b"[CENSUS] battery up expected=1 up=1".to_vec(),
        "the battery row"
    );
}

#[test]
fn a_battery_not_read_or_not_reported_is_missing_and_red() {
    let unread = Facts { battery: Battery::Unread, ..laptop() };
    assert_eq!(seen(&unread, "battery"), Seen::Missing("present, its charge not read"));
    assert!(!Tally::of(&rows(&unread)).green());
    let quiet = Facts { battery: Battery::Unreported, ac_known: false, ..laptop() };
    assert_eq!(seen(&quiet, "battery"), Seen::Missing("no report from the ACPI capsule yet"));
    assert_eq!(seen(&quiet, "ac"), Seen::Missing("adapter state not read"));
    let gone = Facts { acpi_capsule: false, ..laptop() };
    assert_eq!(seen(&gone, "lid"), Seen::Missing("the ACPI capsule is not running"));
    let empty = Facts { battery: Battery::Empty, ..laptop() };
    assert_eq!(seen(&empty, "battery"), Seen::Absent("the battery slot is empty"));
    assert!(Tally::of(&rows(&empty)).green(), "an empty slot is not a fault");
}

#[test]
fn the_desktop_has_no_battery_ac_lid_or_touchpad_and_is_green() {
    let f = desktop();
    for c in ["battery", "ac", "lid"] {
        assert_eq!(seen(&f, c), Seen::Absent("firmware declares none"), "{c}");
    }
    assert_eq!(seen(&f, "pointer"), Seen::Absent("no touchpad declared"));
    assert!(Tally::of(&rows(&f)).green());
}

#[test]
fn the_vm_is_green_on_the_firmware_framebuffer() {
    let f = vm();
    assert_eq!(seen(&f, "display"), Seen::Up, "the firmware framebuffer is a display");
    assert_eq!(seen(&f, "thermal"), Seen::Absent("firmware declares none"));
    assert_eq!(seen(&f, "audio"), Seen::Absent("no audio device"));
    assert!(Tally::of(&rows(&f)).green());
    let no_acpi = Facts { acpi: None, ..vm() };
    assert_eq!(seen(&no_acpi, "battery"), Seen::Absent("no ACPI tables"));
}

#[test]
fn a_device_no_driver_claimed_is_missing_by_its_driver_and_a_held_one_is_not() {
    let nvme = Count { expected: 1, up: 0, unclaimed_by: Some("driver_nvme"), ..Count::default() };
    let f = Facts { storage: nvme, ..laptop() };
    assert_eq!(seen(&f, "storage"), Seen::Missing("driver_nvme"));
    assert_eq!(summary(&f), b"[CENSUS] summary up=9 absent=0 held=1 missing=1 verdict=red");
    let wifi = Count {
        held: 1,
        held_why: Some("no driver for this Realtek Wi-Fi part"),
        ..Count::default()
    };
    let f = Facts { network: wifi, ..laptop() };
    assert_eq!(seen(&f, "network"), Seen::Held("no driver for this Realtek Wi-Fi part"));
    assert!(Tally::of(&rows(&f)).green());
}

/// Every row of every set, as the kernel writes it, reads back field for
/// field, and a report carrying them all passes `check`.
#[test]
fn each_line_reads_back_and_rides_in_a_report() {
    let mut buf = vec![0u8; 16 * 1024];
    let mut r = Report::new(&mut buf, b"b80ae30b3", b"standard-dev", false);
    let mut carried = 0;
    for f in [laptop(), desktop(), vm(), Facts { battery: Battery::Unread, ..laptop() }] {
        for row in rows(&f) {
            let l = line(&row);
            let Some(Census::Row(p)) = parse(&l) else { panic!("{}", String::from_utf8_lossy(&l)) };
            assert_eq!(
                (p.class, p.expected, p.up),
                (row.class.name().as_bytes(), row.expected, row.up)
            );
            assert_eq!(p.why, row.seen.why().map(str::as_bytes));
            assert_eq!(p.state == State::Up, row.seen == Seen::Up);
            assert!(r.census(&l));
            carried += 1;
        }
        let s = summary(&f);
        let Some(Census::Summary(p)) = parse(&s) else { panic!("summary") };
        assert_eq!(p.green, Tally::of(&rows(&f)).green());
        assert!(r.census(&s));
        carried += 1;
    }
    assert!(!r.census(b"[ACPI] power battery=yes charge=57 ac=offline"), "not a census line");
    let (len, cut) = r.finish();
    assert!(!cut);
    let s = check(&buf[..len]).expect("the report reads");
    assert_eq!(s.census, carried);
}

#[test]
fn a_line_that_does_not_hold_together_is_not_carried() {
    for bad in [
        &b"[CENSUS] battery up expected=1 up=1 why=it is up"[..],
        b"[CENSUS] battery missing expected=1 up=0",
        b"[CENSUS] battery maybe expected=1 up=1",
        b"[CENSUS] Battery up expected=1 up=1",
        b"[CENSUS] summary up=1 absent=0 held=0 missing=1 verdict=green",
        b"[CENSUS] battery up expected=1000 up=1",
    ] {
        assert!(parse(bad).is_none(), "{}", String::from_utf8_lossy(bad));
    }
}

#[test]
fn the_rows_come_in_the_classes_order() {
    assert_eq!(rows(&laptop()).map(|r| r.class), rule::CLASSES);
}

/// The HP with an EC that starts late: the ACPI capsule's first pass, and
/// the I2C host behind the touchpad it publishes, come at 7 s, past init's
/// 5 s wait; init spawns the I2C drivers at its next tick and the touchpad
/// is claimed at 8 s. The census does not look before the pass, so it is
/// never said with the touchpad absent: it is said once, at 8 s, green, the
/// touchpad up.
#[test]
fn a_touchpad_whose_host_is_published_at_7_s_is_up_in_the_census() {
    let at = |t: u64| {
        let pointer = match t {
            t if t < 7_000 => Count::default(),
            t if t < 8_000 => {
                Count { expected: 1, unclaimed_by: Some("driver_i2c_hid"), ..Count::default() }
            }
            _ => one(1),
        };
        Facts { pointer, ..laptop() }
    };
    let mut said: Vec<(u64, [Row; 11])> = Vec::new();
    for t in (1_000..=40_000).step_by(1_000) {
        let now = rows(&at(t));
        let look = when::Look {
            said: !said.is_empty(),
            changed: said.last().map(|s| s.1) != Some(now),
            green: Tally::of(&now).green(),
            pass_done: t >= 7_000,
            waited_ms: t,
        };
        if when::say_now(look) {
            said.push((t, now));
        }
    }
    assert_eq!(said.len(), 1, "said once");
    let (t, rows) = said[0];
    assert_eq!(t, 8_000);
    assert_eq!(rows.iter().find(|r| r.class.name() == "pointer").map(|r| r.seen), Some(Seen::Up));
    assert!(Tally::of(&rows).green());
}

/// A pass that never comes holds the census back only until its wait: it
/// is said at 30 s with what is there.
#[test]
fn a_pass_that_never_comes_holds_the_census_until_its_wait() {
    let look = |waited_ms| when::Look {
        said: false,
        changed: true,
        green: true,
        pass_done: false,
        waited_ms,
    };
    assert!(!when::say_now(look(29_999)));
    assert!(when::say_now(look(when::WAIT_MS)));
}

#[path = "../../capsule_about/src/about/data/census.rs"]
mod about;

fn about_rows(f: &Facts) -> Vec<(String, String)> {
    let mut text = Vec::new();
    for r in rows(f) {
        text.extend(line(&r));
        text.push(b'\n');
    }
    text.extend(summary(f));
    text.push(b'\n');
    let mut buf = [0u8; about::VALUE];
    text.split(|b| *b == b'\n')
        .filter_map(|l| {
            let (label, n) = about::row(l, &mut buf)?;
            Some((String::from_utf8_lossy(label).into(), String::from_utf8_lossy(&buf[..n]).into()))
        })
        .collect()
}

/// About's Devices card, from the kernel's own lines: one row a class and
/// the verdict, a missing one with its reason, a held one as present.
#[test]
fn about_shows_each_class_and_why_from_the_kernels_lines() {
    let green = about_rows(&laptop());
    assert_eq!(green.len(), about::ROWS);
    assert_eq!(green[0], ("Battery".into(), "Up".into()));
    assert_eq!(
        green[3],
        (
            "Thermal zones".into(),
            "Present, not read: zones declared; their temperatures are the ACPI capsule's own"
                .into()
        )
    );
    assert_eq!(green[11], ("Census".into(), "Every device up".into()));
    let red = about_rows(&Facts { battery: Battery::Unread, ..laptop() });
    assert_eq!(red[0], ("Battery".into(), "Missing: present, its charge not read".into()));
    assert_eq!(red[11], ("Census".into(), "1 device missing".into()));
    let vm = about_rows(&vm());
    assert_eq!(vm[0], ("Battery".into(), "None".into()));
}

/// MkBootCensus hands back whole lines only: a buffer too small for all of
/// them takes those that fit, never half of one.
#[test]
fn a_short_buffer_takes_whole_lines() {
    let text = b"[CENSUS] battery up expected=1 up=1\n[CENSUS] ac up expected=1 up=1\n";
    assert_eq!(when::whole_lines(text, 1_000), text.len());
    assert_eq!(when::whole_lines(text, 40), 36, "the first line and its newline");
    assert_eq!(when::whole_lines(text, 10), 0);
}
