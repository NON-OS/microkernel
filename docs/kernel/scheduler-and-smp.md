# Scheduler and SMP

How the NONOS kernel starts every CPU in the machine, what it keeps per CPU, and how each CPU decides which process to run next.

## How many CPUs run

Every CPU the firmware enables is started, within the limits below. The build configuration sets `smp` to true in `nonos.toml:20`, which is also the default in `tools/nix/config.nix:127`, and that adds the `nonos-smp` kernel feature to the image's `features` (`tools/nix/config.nix:144`). A kernel built without that feature runs on one CPU.

The number comes from the ACPI MADT. `secondaries` keeps every enabled processor entry and drops duplicates, placeholders, entries past the limit, and APIC ids above `0xFE` while the APIC is in xAPIC mode (`src/smp/topology/detection/probe_x86_64/mod.rs:74-114`). CPUID's count is used only on a machine with no MADT. It prints one line, for example `[SMP] madt aps=3 disabled=0 duplicate=0 invalid=0 unaddressable=0 over_limit=0`. The limit is `MAX_CPUS`, 256 (`src/smp/constants.rs:17`).

The boot CPU registers itself first. `init_bsp` records its APIC id, marks descriptor 0 online, runs the topology detection and fills per-CPU block 0 (`src/smp/init/bsp.rs:24-54`).
