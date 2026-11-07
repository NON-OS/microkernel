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

//! The boot census's rule, pure: what the platform says it has, against
//! what came up, one row a class of device. A row is up, absent (no such
//! hardware, and why), missing (the platform has it and it did not come up,
//! and why) or held (the platform has it and this build does not bring it
//! up by design, and why). The census is green when nothing is missing.
//! `kernel_proofs` and `hwreport_proofs` mount this file.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    Battery,
    Ac,
    Lid,
    Thermal,
    Keyboard,
    Pointer,
    Display,
    Storage,
    Network,
    Usb,
    Audio,
}

pub const CLASSES: [Class; 11] = [
    Class::Battery,
    Class::Ac,
    Class::Lid,
    Class::Thermal,
    Class::Keyboard,
    Class::Pointer,
    Class::Display,
    Class::Storage,
    Class::Network,
    Class::Usb,
    Class::Audio,
];

impl Class {
    pub fn name(self) -> &'static str {
        match self {
            Class::Battery => "battery",
            Class::Ac => "ac",
            Class::Lid => "lid",
            Class::Thermal => "thermal",
            Class::Keyboard => "keyboard",
            Class::Pointer => "pointer",
            Class::Display => "display",
            Class::Storage => "storage",
            Class::Network => "network",
            Class::Usb => "usb",
            Class::Audio => "audio",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seen {
    Up,
    Absent(&'static str),
    Missing(&'static str),
    Held(&'static str),
}

impl Seen {
    pub fn word(self) -> &'static str {
        match self {
            Seen::Up => "up",
            Seen::Absent(_) => "absent",
            Seen::Missing(_) => "missing",
            Seen::Held(_) => "held",
        }
    }

    pub fn why(self) -> Option<&'static str> {
        match self {
            Seen::Up => None,
            Seen::Absent(w) | Seen::Missing(w) | Seen::Held(w) => Some(w),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row {
    pub class: Class,
    pub expected: u8,
    pub up: u8,
    pub seen: Seen,
}

/// What the ACPI capsule last said of the battery, from its fresh report.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Battery {
    /// No fresh report.
    #[default]
    Unreported,
    /// _STA answered: no battery in the slot.
    Empty,
    /// Present, its charge not read (yet).
    Unread,
    /// Present and reporting its charge.
    Charged,
}

/// The devices of one class in the device table: how many, how many a
/// driver holds, how many no driver here takes by design and why, and the
/// driver that did not claim one.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Count {
    pub expected: u8,
    pub up: u8,
    pub held: u8,
    pub held_why: Option<&'static str>,
    pub unclaimed_by: Option<&'static str>,
}

/// What the firmware tables declare: None on a machine without ACPI.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Declared {
    pub battery: bool,
    pub ac: bool,
    pub lid: bool,
    pub thermal_zones: u16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Facts {
    pub acpi: Option<Declared>,
    /// The ACPI capsule was spawned and is alive.
    pub acpi_capsule: bool,
    pub battery: Battery,
    /// The capsule's report carries an adapter state.
    pub ac_known: bool,
    pub lid_known: bool,
    /// The firmware framebuffer the kernel scans out.
    pub boot_framebuffer: bool,
    pub keyboard: Count,
    pub pointer: Count,
    pub display: Count,
    pub storage: Count,
    pub network: Count,
    pub usb: Count,
    pub audio: Count,
}

const NO_ACPI: &str = "no ACPI tables";
const NOT_DECLARED: &str = "firmware declares none";
const NO_CAPSULE: &str = "the ACPI capsule is not running";
const NO_REPORT: &str = "no report from the ACPI capsule yet";

fn row(class: Class, expected: u8, up: u8, seen: Seen) -> Row {
    Row { class, expected, up, seen }
}

/// A row from what the capsule reports, for a device the tables declare.
fn acpi_row(f: &Facts, class: Class, declared: impl Fn(&Declared) -> bool, up: Seen) -> Row {
    match f.acpi {
        None => row(class, 0, 0, Seen::Absent(NO_ACPI)),
        Some(d) if !declared(&d) => row(class, 0, 0, Seen::Absent(NOT_DECLARED)),
        Some(_) if !f.acpi_capsule => row(class, 1, 0, Seen::Missing(NO_CAPSULE)),
        Some(_) => row(class, 1, u8::from(up == Seen::Up), up),
    }
}

fn battery(f: &Facts) -> Row {
    let seen = match f.battery {
        Battery::Charged => Seen::Up,
        Battery::Unreported => Seen::Missing(NO_REPORT),
        Battery::Unread => Seen::Missing("present, its charge not read"),
        Battery::Empty => Seen::Absent("the battery slot is empty"),
    };
    acpi_row(f, Class::Battery, |d| d.battery, seen)
}

fn from_table(class: Class, c: &Count, none: &'static str) -> Row {
    match (c.expected, c.up >= c.expected) {
        (0, _) if c.held > 0 => {
            row(class, 0, 0, Seen::Held(c.held_why.unwrap_or("no driver here")))
        }
        (0, _) => row(class, 0, 0, Seen::Absent(none)),
        (n, true) => row(class, n, c.up, Seen::Up),
        (n, false) => row(class, n, c.up, Seen::Missing(c.unclaimed_by.unwrap_or("not claimed"))),
    }
}

pub fn rows(f: &Facts) -> [Row; 11] {
    let on = |b: bool, why: &'static str| if b { Seen::Up } else { Seen::Missing(why) };
    let thermal = match f.acpi {
        None => row(Class::Thermal, 0, 0, Seen::Absent(NO_ACPI)),
        Some(d) if d.thermal_zones == 0 => row(Class::Thermal, 0, 0, Seen::Absent(NOT_DECLARED)),
        Some(d) => row(
            Class::Thermal,
            d.thermal_zones.min(255) as u8,
            0,
            Seen::Held("zones declared; their temperatures are the ACPI capsule's own"),
        ),
    };
    let mut display = from_table(Class::Display, &f.display, "no display device");
    if f.boot_framebuffer && display.seen != Seen::Up {
        display = row(Class::Display, display.expected.max(1), display.up.max(1), Seen::Up);
    }
    [
        battery(f),
        acpi_row(f, Class::Ac, |d| d.ac, on(f.ac_known, "adapter state not read")),
        acpi_row(f, Class::Lid, |d| d.lid, on(f.lid_known, "lid state not read")),
        thermal,
        from_table(Class::Keyboard, &f.keyboard, "no PS/2 or virtual keyboard; USB ones are usb's"),
        from_table(Class::Pointer, &f.pointer, "no touchpad declared"),
        display,
        from_table(Class::Storage, &f.storage, "no storage controller"),
        from_table(Class::Network, &f.network, "no network device"),
        from_table(Class::Usb, &f.usb, "no USB host"),
        from_table(Class::Audio, &f.audio, "no audio device"),
    ]
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    pub up: u8,
    pub absent: u8,
    pub missing: u8,
    pub held: u8,
}

impl Tally {
    pub fn of(rows: &[Row]) -> Tally {
        let mut t = Tally::default();
        for r in rows {
            match r.seen {
                Seen::Up => t.up += 1,
                Seen::Absent(_) => t.absent += 1,
                Seen::Missing(_) => t.missing += 1,
                Seen::Held(_) => t.held += 1,
            }
        }
        t
    }

    pub fn green(&self) -> bool {
        self.missing == 0
    }
}

fn dec(n: u8, put: &mut dyn FnMut(&[u8])) {
    let digits = [b'0' + n / 100, b'0' + n / 10 % 10, b'0' + n % 10];
    let from = if n >= 100 {
        0
    } else if n >= 10 {
        1
    } else {
        2
    };
    put(&digits[from..]);
}

/// `[CENSUS] <class> <up|absent|missing|held> expected=<n> up=<n>[ why=<reason>]`
pub fn render(r: &Row, put: &mut dyn FnMut(&[u8])) {
    put(b"[CENSUS] ");
    put(r.class.name().as_bytes());
    put(b" ");
    put(r.seen.word().as_bytes());
    put(b" expected=");
    dec(r.expected, put);
    put(b" up=");
    dec(r.up, put);
    if let Some(why) = r.seen.why() {
        put(b" why=");
        put(why.as_bytes());
    }
}

/// `[CENSUS] summary up=<n> absent=<n> held=<n> missing=<n> verdict=<green|red>`:
/// the line the boot matrix and tools/nonos-bringup-gate read.
pub fn render_summary(t: &Tally, put: &mut dyn FnMut(&[u8])) {
    put(b"[CENSUS] summary up=");
    dec(t.up, put);
    put(b" absent=");
    dec(t.absent, put);
    put(b" held=");
    dec(t.held, put);
    put(b" missing=");
    dec(t.missing, put);
    put(if t.green() { b" verdict=green" } else { b" verdict=red" });
}
