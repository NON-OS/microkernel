// NONOS Operating System (AGPL-3.0-or-later)
//! TSC frequency from CPUID and from reference clocks, on register values
//! shaped like the parts named (ratios illustrative, encodings exact).

use crate::tsc_math::*;

fn leaf0(id: &[u8; 12]) -> (u32, u32, u32) {
    let w = |i: usize| u32::from_le_bytes([id[i], id[i + 1], id[i + 2], id[i + 3]]);
    (w(0), w(4), w(8))
}

#[test]
fn vendors_are_told_apart() {
    let (b, d, c) = leaf0(b"GenuineIntel");
    assert_eq!(vendor_from_leaf0(b, d, c), Vendor::Intel);
    let (b, d, c) = leaf0(b"AuthenticAMD");
    assert_eq!(vendor_from_leaf0(b, d, c), Vendor::Amd);
    let (b, d, c) = leaf0(b"KVMKVMKVM\0\0\0");
    assert_eq!(vendor_from_leaf0(b, d, c), Vendor::Other);
}

#[test]
fn family_and_model_include_the_extended_fields() {
    // Celeron N4120 (Gemini Lake Refresh): 0x000706A8 -> family 6 model 0x7A.
    assert_eq!(family_model(0x0007_06A8), (6, 0x7A));
    // Ryzen 7 5800U: 0x00A50F00 -> family 0x19 model 0x50.
    assert_eq!(family_model(0x00A5_0F00), (0x19, 0x50));
}

#[test]
fn gemini_lake_reports_its_crystal_directly() {
    // Goldmont Plus enumerates a 19.2 MHz crystal in CPUID.15H ECX.
    let hz = cpuid_tsc_hz(Vendor::Intel, 6, 0x7A, Some((1, 0x3E, 19_200_000)), Some(1100));
    assert_eq!(hz, Some(1_190_400_000));
}

#[test]
fn skylake_client_derives_the_crystal_from_leaf_0x16() {
    // i5-6200U: CPUID.15H EAX=2 EBX=0xC8 ECX=0; CPUID.16H EAX=2400 (MHz).
    let hz = cpuid_tsc_hz(Vendor::Intel, 6, 0x4E, Some((2, 0xC8, 0)), Some(2400));
    assert_eq!(hz, Some(2_400_000_000));
}

#[test]
fn denverton_uses_its_fixed_25_mhz_crystal() {
    let hz = cpuid_tsc_hz(Vendor::Intel, 6, 0x5F, Some((1, 0x58, 0)), None);
    assert_eq!(hz, Some(2_200_000_000));
}

#[test]
fn amd_and_ratio_less_parts_are_measured_instead() {
    // AMD never answers from CPUID, whatever the leaves hold.
    assert_eq!(cpuid_tsc_hz(Vendor::Amd, 0x19, 0x50, Some((1, 100, 25_000_000)), None), None);
    // Intel with no ratio (Haswell-era leaf 0x15 of zeros) or no leaf 0x15.
    assert_eq!(cpuid_tsc_hz(Vendor::Intel, 6, 0x3C, Some((0, 0, 0)), Some(3400)), None);
    assert_eq!(cpuid_tsc_hz(Vendor::Intel, 6, 0x3C, None, None), None);
    // Ratio but neither crystal nor leaf 0x16: nothing to multiply.
    assert_eq!(cpuid_tsc_hz(Vendor::Intel, 6, 0x8E, Some((2, 0xC8, 0)), None), None);
}

#[test]
fn the_pm_timer_wraps_at_24_or_32_bits() {
    assert_eq!(pm_timer_delta(0x00FF_FFF0, 0x0000_0010, false), 0x20);
    assert_eq!(pm_timer_delta(0xFFFF_FFF0, 0x0000_0010, true), 0x20);
    assert_eq!(pm_timer_delta(100, 300, false), 200);
}

#[test]
fn a_reference_window_gives_the_tsc_rate() {
    // 50 ms of PM timer at 3.579545 MHz against 150e6 TSC ticks = 3 GHz.
    let ticks = PM_TIMER_HZ * 50 / 1000;
    let hz = hz_from_reference(150_000_000, ticks, PM_TIMER_HZ).unwrap();
    assert!((2_999_000_000..=3_001_000_000).contains(&hz), "{hz}");
    assert_eq!(hz_from_reference(1, 0, PM_TIMER_HZ), None);
}

#[test]
fn a_dead_reference_gives_up_in_bounded_time() {
    // 50 ms window, 10 GHz ceiling: at most 1e9 TSC ticks, under a second on
    // any part the kernel accepts.
    assert_eq!(reference_timeout_ticks(50, 10_000_000_000), 1_000_000_000);
}
