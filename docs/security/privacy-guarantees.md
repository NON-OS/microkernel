# Privacy guarantees

What NONOS promises about leaving no trace and leaking nothing. Each promise names its proof and is marked at the level the proof reaches:

- **booted**: shown on a booted image, with the command and its output named;
- **host-tested**: a test runs the real source on a build machine, and a mutant that removes the control is caught;
- **design**: the code is written to hold it, and no test checks it yet.

No claim is rounded up. Where a promise does not hold, this page says so and names the open work.

The image the booted rows refer to is the `standard` profile's development twin at commit 308615832, sealed with throwaway keys (`tools/nonos-dev-image --profile standard`, image sha256 `e1b3cff8…5cd011288`). It carries the shipped feature set, with path-only attestation and the development loader. Like `standard`, it has no capsule serial output (see [the feature inventory](../build/feature-inventory.md)). Section 7 says how it was booted, and what the boots did and did not exercise.

## 1. Nothing persists unless the person chose it

| claim | level | proof |
|---|---|---|
| A boot keeps nothing until first-boot setup records the choice. The policy field `Persistent` starts `false` (`userland/capsule_policy/src/store/defaults/store.rs`), and only setup and settings may write it (`capsule_policy/src/server/handle_set.rs`). | design | none |
| On a boot that keeps nothing, vfs refuses to keep a file (`EACCES`). Zeros, an empty file included, go to the disk only as the replacement of a record the disk already holds, at its own length; a name the disk does not hold is never added (`keep`, `userland/capsule_vfs/src/server/handlers/persist_rule.rs`; `withdraw`, `userland/capsule_vfs/src/blk/store_withdraw.rs`). A replacement writes the new bytes to a free extent, repoints the entry, then zeroes the extent the record left, so a withdrawn or replaced record's old bytes leave the disk, not only the table (`userland/capsule_vfs/src/blk/store_replace.rs`). | host-tested | `userland/fs_proofs`: `vfs_disk::tests_withdraw` (the disk is unchanged sector for sector when nothing may be written), `tests_withdraw_kept` (a cut at any sector leaves the old record or the zeros), `tests_erase` (none of a withdrawn or replaced record's old bytes is left anywhere on the disk), and `vfs_persist_rule_tests`. Mutants: withdraw appending an absent name; the rule passing any byte; the old extent left unzeroed. Each is caught. |
| On a boot that keeps nothing, removing a store entry is refused and an uninstall drops only the RAM copy (`store_remove.rs`, `store_uninstall.rs` in `userland/capsule_vfs/src/server/handlers/`). | design | none: the boot in §7 removes nothing |
| An idle first boot writes nothing. With the disk driver up for 25 minutes, the data disk is byte for byte as it went in, and every changed EFI variable is the firmware's own. | booted | `scripts/amnesic_boot_check.sh` (§7) |
| There is no swap or page-out path. | design | no swap, page-out or swapon code exists in `src/` |
| A panic writes no crash record: the handler writes only to serial and the screen (`src/boot/panic/handler.rs`), and nothing calls the kernel's `set_variable` (`src/arch/x86_64/uefi/api.rs`). | design | none |
| The DNS cache lives in RAM only. | design | `capsule_net_dns/src/state.rs`; the image does not ship net.dns, and net.core's resolver keeps no cache on disk |
| Logs live in RAM and on the serial line, and in no file. | design | `src/log/backend/` has a RAM ring, serial and VGA backends, and no file backend |

**Not yet closed:**

- **Persistence after a failed setup restore.** On an installed disk where the setup record fails to restore, `Persistent` is false, but a model import still lands on the disk volume. The kernel picks the volume by whether a data plan exists (`src/fs/blockfs_volume/open_machine.rs`). It should also follow `Persistent`.
- **Removed payloads stay on disk.** Removing a store entry frees its table slot, but the payload bytes stay on disk until they are overwritten (`blk/store_remove.rs`).
- **The payment meter's key.** It keeps its sealed key at `/data/meter.key` on a persistent install (`userland/capsule_payment/README.md`). It is not in the default image.

## 2. RAM is wiped at the end of a boot

| claim | level | proof |
|---|---|---|
| Every power-off and reboot goes through `zerostate::terminate`, which stops the other CPUs, wipes, then powers off. `arch::power::enter` is crate-private, and `terminate` is its only caller. | design, and a model | `verification/lean/Nonos/ZeroState.lean`: `off_was_wiped`, `wipe_covers_every_region` (CI: `lean.yml`). The model covers the region list the code names, and no more. |
| The shutdown wipe covers: device DMA grant frames, each process's code and VMAs, the kernel stacks other than the running one, the file caches, the key vault, the RAM log (if its lock is free) and then the whole heap (`zerostate_shutdown_wipe`, `src/security/hardening/memory_sanitization/api.rs`). | design; parts host-tested | `userland/kernel_proofs`: `dma_wipe`, `ipc_wipe` |
| A panic stops the other CPUs and waits, bounded, for each to report itself halted (`smp::others_halted`). It reads the soft-off parameters, zeroes the kernel heap minus its own stack and the kernel image, and powers off. Once the wipe begins it takes no lock, makes no allocation and writes no log (`src/security/zerostate/panic_wipe.rs`). | host-tested | `userland/kernel_proofs` `panic_plan` checks byte for byte that no kept range is wiped and every other heap byte is, the vault's place included, over 400 generated layouts. Mutants: wiping through the kept range; dropping the part above it. Each is caught. No boot yet shows an injected panic ending in power-off. |
| A capsule cannot reset or power off the machine around the wipe. The PS/2 grant refuses the i8042 reset commands, 0xF0–0xFF and 0xD1, on port 0x64 (`src/hardware/broker/pio/access/reset_rule.rs`). The FADT's PM1 control, sleep control and reset registers are refused to an OperationRegion and to a PIO grant (`src/hardware/broker/acpi/rule/power.rs`). Port 0x92 and 0xCF9 were reserved before this change. | host-tested | `kernel_proofs` `pio_reset` (mutant: the 0xF0 range dropped); `acpi_region` `tests_ports` and the sweep (mutant: the power check dropped). Each is caught. |

**Not yet closed:**

- **Panic wipe coverage.** A panic does not wipe process memory: finding its frames takes the process table and pager locks, which the panic may hold. Writing allocation-free, try-lock walks of them is the next step.
- **Shutdown wipe gaps.** The shutdown wipe does not reach frames freed without zeroing through `phys::free`, the kernel's own `.data`/`.bss` beyond the log, page tables, the framebuffer, or the running stack. The session RAM volume is erased only cryptographically: its key is in the heap, and the heap is wiped.
- **RAM log under contention.** The RAM log is skipped when its lock is held.
- **No booted proof.** No boot dumps guest RAM after a wipe.

## 3. Network: anonymity networks where the design says so

| claim | level | proof |
|---|---|---|
| The image sends no DNS, NTP, mDNS, LLMNR, SSDP or NetBIOS packet and no IPv6, and talks only to peers it chose: pinned bootstrap hosts, and relays named by a consensus a pinned authority sent. | booted | `scripts/no_telemetry_qemu_capture.sh` judged by `tools/nonos-pcap-egress` (§7): 2283 guest frames, every peer pinned |
| No name is looked up in the clear unless the chosen network is Direct. Off Direct, net.sockets refuses a connect by name on any socket, and net.dns is never asked (`host_target`, `userland/capsule_net_sockets/src/server/handlers/connect/host_target.rs`). Each client (the browser, `nonos_std`, the terminal, model fetch) checks the route before it asks, too. | host-tested | `userland/capsule_linux_proofs` `mixnet_name_tests` (mutant: the Direct check dropped, caught) |
| The wallet's RPC never goes Direct (`Route::for_wallet`, `userland/nonos_route_link/src/pick.rs`). Installs go over Anyone by default (`Route::for_installs`). | design | none |
| The image has no time sync, so it sends no NTP. `net.ntp` is not spawned beside `net.core` (`spawn_legacy_stack`). | design | the capture (§7) |
| Telemetry, update checks and captive-portal probes do not exist. | design | grep over `userland/` and `src/` for telemetry, captive, connectivity and update-check endpoints finds none; the capture (§7) |

**Not yet closed:**

- **One Network capability.** A capsule holding it can open a direct socket through net.core or net.sockets whatever route was chosen; the route rule lives in each client (`nonos_route_link`). Splitting it into an anonymous and a direct capability is the fix.
- **Desktop shell's Network grant.** The shell holds Network only to read the DHCP state.
- **Bootstrap before consent.** The Nym and Anyone clients reach their directories and a gateway at boot, before the person has chosen anything, so the local network sees a machine using both (`capsule_net_nym/src/server/runner.rs`, `capsule_net_anon/src/server/idle.rs`).

## 4. Identifiers

| claim | level | proof |
|---|---|---|
| Every NIC transmits from a fresh random, locally administered unicast MAC on each bring-up, never the burned-in one (`nonos_mac::apply`, `userland/nonos_mac/src/local.rs`). | host-tested; booted for virtio-net | `userland/nonos_mac/src/tests.rs` and the driver proofs crates; on the boot in §7 the guest sent from `7e:4e:99:54:18:58`, and the NIC's own `52:54:00:12:34:56` only in the firmware's three frames before NONOS ran |
| The image sends no hostname: no DHCP option 12, 60, 77 or 81, and no mDNS. | booted | the capture's `dhcp-id` and `discovery` rules (§7) |
| DHCP asks for what Windows 10 and 11 ask for, in their order (`PARAMETER_REQUEST_LIST`, `userland/capsule_net_core/src/iface/dhcp/request_list.rs`), not smoltcp's `[1, 3, 6]`, which marked a NONOS machine on any LAN. | host-tested; booted | `userland/net_core_proofs` `dhcp::tests` takes net.core's first DHCPDISCOVER off a smoltcp link (mutant: the list not set, caught); the capture in §7 shows the list on the wire |
| Scripts in the browser see UTC: `localtime_r` is stubbed to zero (`userland/nonos_qjs/vendor/stubs.c`), and no client sends a `Date` header. | design | none |
| Scripts in the browser get no font list, screen size or `hardwareConcurrency`. | design | the DOM prelude (`userland/nonos_qjs/vendor/dom_prelude_1.inc`) exposes none of them |

**Not yet closed:**

- **TLS fingerprint.** The TLS ClientHello is NONOS's own: two cipher suites, five extensions in a fixed order and no ALPN (`userland/nonos_tls/src/client_hello.rs`). Its JA3/JA4 marks every NONOS client to an exit or a server.
- **Browser headers.** The browser claims Firefox 128 (`capsule_browser/src/browser/http/request.rs`), while its Accept header, HTTP/1.1 and TLS do not match Firefox.
- **Self-naming User-Agents.** The Linux installer, model fetch, `nox` and Nym's directory fetch name themselves `nonos…`.
- **DHCP residue.** The request matches Windows on option 55, but not on everything. NONOS sends no hostname, vendor class or FQDN, which Windows does, and smoltcp adds option 57 and fixes the option order.
- **The Nym validator's name is sent in the clear.** The Nym client's directory fetch sends `validator.nymtech.net` as its TLS server name, in the clear, to 92.39.63.14:443 (52 times in the §7 capture). The host is pinned by address and never resolved, but the name shows anyone on the path that this machine uses Nym. Encrypted Client Hello would hide it, and `nonos_tls` has none (`client_hello.rs`).

## 5. Keys

| claim | level | proof |
|---|---|---|
| A capsule's keys are generated, kept and used in the keyring capsule. The keyring answers each caller only for keys it owns, by the pid the kernel stamps; it never trusts a pid from the payload (`resolve_caller`, `userland/capsule_keyring/src/server/caller.rs`). | design | none |
| Reaching the keyring takes the Keyring capability: the kernel admits a sender to the `keyring` endpoint only with it (`KEY_SERVICES`, `src/services/registry/policy.rs`). | host-tested | `kernel_proofs` `service_policy_tests` (mutant: the keyring branch dropped, caught) |
| Keys outside the keyring are wiped when done. A TLS connection's secrets and record keys live in one heap block a `TrafficKeys` points to, written there in place by the key schedule, so moving the keys copies a pointer, never a secret; the block is zeroed on drop (`userland/nonos_tls/src/traffic_keys.rs`). The flight's two private keys are boxed the same way, wiped on drop and at every connection's settle (browser, wallet, terminal), and the ECDHE secret, every HKDF block, the MAC's output and the Finished key are wiped once used. What is not wiped: the internal state of the `hmac` 0.12 and `sha2` 0.10 crates, which this version cannot zero. net.nym's identity seed is wiped on each copy's drop, in the request that installs it, and with every request's bytes once the request is served (`server/serve_one.rs`). The identity itself is fresh each boot and held for the whole boot by design: net.nym does not clear it, and only the shutdown wipe does. The gateway key lives in one slot and is never copied out: each frame borrows it under the lock (`with_gateway_key`), and the Gateway record copied into the table and every run no longer carries it. It is wiped where it lies when its link is lost (`gateway_lost`), or when a gateway is set on any other connection. The handshake's Diffie-Hellman result and extracted key are wiped, as are every HMAC and HKDF buffer and the ephemeral on drop, whatever the outcome. net.anon's client key has a wiping Drop. | host-tested: the TLS keys' block and move, the flight's keys, the identity's drop, the key slot through `Table::set_gateway` and the clear `gateway_lost` makes, and the HKDF against a reference. Source-checked: where the handshake, establish, set_identity and serve_one wipe. | `tls_proofs` `key_wipe` and `wipe_glue`, and the RFC 8448 schedule vectors through the reworked schedule; `nym_reply_proofs` `secret_wipe_tests`, `key_slot_tests`, `hkdf_tests`, `wipe_glue_tests`. Mutants: an empty Drop, each wipe removed, the key kept across links, a lost link keeping its key, a borrow with no key held. Each is caught. |
| A locked keyring entry is marked locked and stays in RAM until the process ends; the shutdown wipe takes it. | design | `userland/capsule_keyring/src/store/lock.rs` |

**Not yet closed:**

- **capsule_crypto.** `x25519_public` leaves its local copy of the private key unwiped.
- **The payment meter's MAC key.** It is held for the whole boot and never wiped.

## 6. Capabilities: no capsule broader than it needs

| claim | level | proof |
|---|---|---|
| Every capability a manifest grants is one the capsule's code can be shown to use, except three. CoreExec is granted to attest, prove-fetch and toolkit, and every process holds CoreExec anyway (`AMBIENT_CAPS`, `src/process/core/table/ambient.rs`), so those grants widen nothing. | host-tested | `python3 scripts/cap_audit.py`: `144 capsules, 3 granted bits with no evidence`; `--selftest`. On the boot in §7 the kernel logs every spawn's mask (`[SPAWN] … caps=`). The keyring came up with `0x10000038`, so it holds the Keyring bit its own endpoint asks of senders. Login and the wallet start on demand and did not start in that boot. |
| virtio-gpu holds no Pio (it reaches both virtio layouts through a memory BAR), and login holds no display query (it asks the compositor). | host-tested | the same audit |

The manifests these change are re-signed by the next release seal: keyring, login, payment, wallet_nonos and virtio-gpu.

## 7. Booted evidence

Both runs boot the image named at the top under QEMU 11.1.1 with OVMF, on a q35 machine with an Intel IOMMU (`--lane iommu=intel`). A release image refuses device DMA that no IOMMU confines, and without one its disk and network drivers do not come up at all. QEMU had no KVM, so it emulated the CPU (TCG): 8 GiB, 4 CPUs, virtio-net on user-mode networking, virtio-blk, virtio-vga, xHCI and HDA. Each run lasted 25 minutes, and nobody answered first-boot setup. The exact QEMU command lines are in the evidence folder.

The evidence is in [`privacy-evidence/2026-10-24/`](privacy-evidence/2026-10-24/).

**Network** (`scripts/no_telemetry_qemu_capture.sh` with `NONOS_QEMU_LANES=iommu=intel`): PASS.

- 2894 frames were captured: 2283 from the guest, 3 from the firmware before NONOS ran, and the rest from the host.
- Every peer the guest reached is pinned in the image:
  - the five Nym bootstrap gateways on :9000;
  - Nym's directory API at 92.39.63.14:443;
  - the seven Anyone directory authorities on :9230.
- There was no DNS, NTP, discovery packet or IPv6, and no DHCP option naming the machine.
- The DHCP parameter request list was Windows'.
- `egress-verdict.txt` is the tool's output, `egress-report.json` its report, `egress.pcap` the capture and `egress-serial.log` the serial console.

**Network, on other NICs** (the driver lanes, `docs/build/feature-inventory.md`): the same capture with an emulated Intel NIC in place of virtio-net. The e1000e lane passed with 2313 guest frames and the igb lane with 2307, every peer pinned. The usb-net lane is `EMPTY`, not a pass: the guest sent nothing over it. Each lane's report and serial console are in the evidence folder as `lane-<A|B|C|E>-report.json` and `lane-<…>-serial.log`, with the two NIC captures as `lane-A-e1000e.pcap` and `lane-B-igb.pcap`. runs A to C booted the image named at the top; run E booted `2f423b3b…`, commit e90c51abf.

**Amnesic** (`scripts/amnesic_boot_check.sh` with the same lane): PASS.

- The data disk's sha256 was `1815711f…7b9b8310` before and after the boot.
- `driver.virtio_blk0` came up and stayed up (`amnesic-serial.log`), so the disk could have been written.
- 32 EFI variables changed, all under the firmware's own GUIDs and names, and 0 were NONOS's (`amnesic-efi-vars.txt`).

**What these boots do not show:**

- They are idle first boots. Nobody chose to keep data, opened an app or browsed. The persist, remove and uninstall gates on such a boot are host-tested (§1); no boot exercised them through the UI.
- The Anyone client reached its authorities, but no relay appears in the capture, so the relay path, and the consensus-learned allowlist with it, was not exercised.
- The Nym client reached its gateways. Whether a mixnet session carried traffic is not shown.
- Real hardware is not covered: firmware differs, Wi-Fi drivers probe for SSIDs, and real NICs replace the emulated ones.
- No RAM was dumped. The wipes in §2 are host-tested and modelled, not observed.
