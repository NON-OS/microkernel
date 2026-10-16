# Design principles: Refuse rather than fall back

Part of [Design principles](design-principles.md).

## Refuse rather than fall back

When something cannot be done safely, the code refuses and says why, instead of
taking a weaker path.

- No route beats the wrong route. A chosen network that is not running gives
  `Down`, never `Direct` (`pick` in
  `userland/nonos_route_link/src/pick.rs:125-139`). The wallet's chain reads go
  over Nym or Anyone even when Direct is chosen (`private_only` in
  `userland/nonos_route_link/src/pick.rs:86-100`).
- No factory address on the wire. The e1000 driver fails when it has no
  randomness for a station address, rather than fall back to the address in its
  EEPROM (`draw` in
  `userland/capsule_driver_e1000/src/init/station_address.rs:17-32`).
- No clock sync that names the machine. net.ntp asks a time server only when
  Direct is chosen (`step` in `userland/capsule_net_ntp/src/decide.rs:17-45`).
  No image built from a build profile starts it: init has `spawn_ntp` only when
  `nonos-capsule-net-core` is off
  (`src/userspace/init/spawn_plan/network/mod.rs:28-29`), and no build profile
  turns on `nonos-capsule-net-ntp` (`tools/nix/config.nix`). The one kernel that
  carries it, the `nonos-mk-ntp-test` smoke test, has no [policy
  store](glossary-peer-list.md#policy-store), so net.ntp there cannot read Direct and asks
  nothing.
- No guessed CPU. The kernel runs on every core, and a CPU whose APIC id is in
  no descriptor halts rather than run as the boot CPU with another CPU's process
  and authority (`cpu_id` in `src/smp/cpu_id.rs:32-43`).
- No guessed Linux call. A call the Linux personality does not serve answers
  ENOSYS and names itself on the log (`unserved` in
  `userland/capsule_linux/src/linux/serve/unserved.rs:21-41`).

## Small files, and mod.rs only declares

The house rule is a file of at most 75 lines that does one thing, and a `mod.rs`
that only declares modules and re-exports them; `verification/ASSUMPTIONS.md`
calls it the 75-line rule. `nonos-ci/run-static-checks.sh` enforces it for the
bootloader: a bootloader `mod.rs` with anything but declarations fails
(`boot_mod_bodies` at `nonos-ci/run-static-checks.sh:2217-2237`), and so does
one past 75 lines (`boot_mod_oversize` at
`nonos-ci/run-static-checks.sh:2240-2246`).

In the kernel it is a convention, not a check. 790 of the 5,761 Rust files under
`src/` are longer than 75 lines, and 36 of its 841 `mod.rs` files define a
function. Counted from the repository root:

```sh
find src -name '*.rs' -print0 | xargs -0 wc -l | awk '$2 != "total" && $1 > 75' | wc -l      # 790
find src -name mod.rs | wc -l                                                                # 841
grep -lE '^\s*(pub(\([a-z]+\))? )?(const )?(unsafe )?fn ' $(find src -name mod.rs) | wc -l    # 36
```
