# rtl8822ce_proofs

Host proofs for `capsule_driver_rtl8822ce`. The crate mounts the driver's
own source by `#[path]` (`src/mounts.rs`, `src/serve/`), and the shared
RTL8821CE files the driver mounts, so what is tested is what the capsule
compiles. The RTL8821CE proofs' libc shim, clock and power-sequence mock
are reused, not copied.

`src/model.rs` is a modelled register file: bytes read back what was
written, every write is logged with its width, and a few registers answer
as the hardware does (`src/model/hooks.rs`: the efuse read-out, the DBI
and LLT flags, the LTE-coex window, the DAC sample register). The tests
(`src/tests/`, one file a subject) check each rule against rtw88 v6.12
and the bounds the driver keeps (efuse map sizes, calibration rounds and
reads, the TX power cap, the log line length).

`verification/host_mutants/rtl8822ce.json` lists one mutant per rule or
bound; `tools/nonos-host-mutants` runs them and each must fail a test here.

    cargo test
