// NONOS Operating System (AGPL-3.0-or-later)
//! Each failure names its stage, step and detail.


use crate::gen3::bringup::BootError;
use crate::gen3::dev::WaitError;
use crate::gen3::outcome::*;
use crate::gen3::select::Refusal;
use crate::gen3::start::StartError;
use crate::gen3::up::UpError;

#[test]
fn each_failure_names_its_stage_step_and_detail() {
    let cases = [
        (Failure::Window, STAGE_DEAD_MMIO, STEP_WINDOW, 0),
        (Failure::DeadMmio, STAGE_DEAD_MMIO, STEP_DEAD_MMIO, 0),
        (Failure::Start(StartError::NotReady), STAGE_POWER_FAILED, STEP_START, 1),
        (Failure::Refused(Refusal::NotSoDevice(0x2725)), STAGE_NO_AIR_PATH, STEP_REFUSED, 0x1_2725),
        (Failure::Refused(Refusal::RfNotBundled(0x105)), STAGE_NO_AIR_PATH, STEP_REFUSED, 0x3_0105),
        (Failure::Image, STAGE_FIRMWARE_FAILED, STEP_IMAGE, 0),
        (Failure::NoDma, STAGE_NO_DMA, STEP_NO_DMA, 0),
        (Failure::Boot(BootError::Start(StartError::ClockNotReady)), STAGE_POWER_FAILED, STEP_BOOT, 0x12),
        (Failure::Boot(BootError::NoNicAccess), STAGE_POWER_FAILED, STEP_BOOT, 0x70),
        (Failure::Boot(BootError::NoAliveCause), STAGE_FIRMWARE_FAILED, STEP_BOOT, 0x30),
        (Failure::Boot(BootError::NotAlive(0xDEAD)), STAGE_FIRMWARE_FAILED, STEP_BOOT, 0x5_DEAD),
        (Failure::Boot(BootError::Pnvm(WaitError::TimedOut)), STAGE_FIRMWARE_FAILED, STEP_BOOT, 0x61),
        (Failure::Up(UpError::Command(0x0C, 0x02, WaitError::TimedOut)), STAGE_FIRMWARE_FAILED, STEP_UP, 0x010C_0201),
        (Failure::Up(UpError::BadNvm), STAGE_FIRMWARE_FAILED, STEP_UP, 0x0300_0000),
        (Failure::Lost, STAGE_FIRMWARE_FAILED, STEP_LOST, 0),
    ];
    for (f, stage, step, detail) in cases {
        assert_eq!((f.stage(), f.step(), f.detail()), (stage, step, detail), "{f:?}");
        assert!(!f.text().is_empty());
    }
    // The stage codes are the client's: Ready 0, PowerFailed 2, DeadMmio 3,
    // FirmwareFailed 4, NoDma 5, NoAirPath 8.
    assert_eq!((STAGE_READY, STAGE_POWER_FAILED, STAGE_DEAD_MMIO), (0, 2, 3));
    assert_eq!((STAGE_FIRMWARE_FAILED, STAGE_NO_DMA, STAGE_NO_AIR_PATH), (4, 5, 8));
}
