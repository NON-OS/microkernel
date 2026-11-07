# Fuzzing

The fuzz targets live beside the code they fuzz, in each crate's own `fuzz/`
directory (`cargo fuzz`, libFuzzer, AddressSanitizer on by default). This
directory holds the list CI fuzzes.

## Which targets CI runs

`targets.json` names every fuzz target in the tree, under `ci` (fuzzed by
`.github/workflows/fuzz.yml`) or `not_in_ci`. `seed: true` runs the crate's
`fuzz/seed.py --out corpus/<target>` before the first run.
`nonos-ci/fuzz_targets.py` builds the workflow's matrix from the `ci` list and
fails when a `[[bin]]` in any tracked `*/fuzz/Cargo.toml` is missing from the
file or a listed one no longer exists, so the list cannot drift from the tree.
`nonos-ci/fuzz_targets.py --check` runs that check alone.

The nightly run gives each target 30 minutes; a manual dispatch picks the
length, at most 320 minutes. The corpus of each target is kept in the Actions
cache between runs.

## What a run leaves

libFuzzer writes a failing input (`crash-*`, `leak-*`, `timeout-*`, `oom-*`)
under `<crate>/fuzz/artifacts/<target>/`. Any of them fails the job with an
error naming the file and the command that replays it:

```
cd <crate>/fuzz && cargo fuzz run <target> artifacts/<target>/crash-<hash>
```

Each target also writes `fuzz-out/<slug>.json` (status, run length,
executions and executions per second, coverage, features, corpus size before
and after, crash inputs), read from libFuzzer's final stats by
`nonos-ci/fuzz_record.py`. The summary job joins them into one table.

## Running one by hand

```
bash nonos-ci/fuzz-run.sh userland/kernel_proofs syscall_gate 10 true
```

## The parsers of untrusted bytes and their targets

| input | reader | target |
|---|---|---|
| syscall number, capability bits, pointer arguments | `src/syscall/abi` decode, `src/syscall/contract/cap_table`, `src/usercopy/policy.rs` | `userland/kernel_proofs` `syscall_gate` |
| MkPeerReadRows's row, count and stride | `src/process/foreign/peer_rows_shape.rs` `Rows` (the layout the syscall checks before any guest page) | `userland/kernel_proofs` `peer_rows` |
| MkPeerPin's surface shape and the guest pages a walk found | `src/kernel_core/surface_registry/pin/pin_rule.rs` `pin_pages`, `pin_frames`, `in_grant` (the shape, then each page: mapped, the guest's own, never executable) | `userland/kernel_proofs` `peer_pin` |
| A process's mmap allocator under any run of reserves and releases, a surface view's release among them | `src/process/mmap_va.rs` `reserve`, `reserve_guarded`, `release`, `release_guarded` (a view's release gives back only a live guarded range of exactly its base and length; no range or guard handed out overlaps another) | `userland/kernel_proofs` `mmap_va_views` |
| A guest's pins under any run of maps, pins, unpins, reprotects, unmaps in whole or in part, exits and the compositor's attaches and releases | `src/kernel_core/surface_registry/pin/unmap_rule.rs` `on_unmap`, `pin_rule.rs` `pin_frames`, `register_rule.rs`, `share/rights_rule.rs` and `rights_attach.rs`, in the kernel_proofs pin model (no view reads a freed or reused frame, every view is read-only, every frame the registry keeps is a live surface's or listed to free, and none is held once every view and guest is gone) | `userland/kernel_proofs` `pin_world` |
| net.core LISTEN, ACCEPT, CLOSE and the reap over listeners | `userland/capsule_net_core/src/listeners/table.rs` `Listeners` and `listeners/gate.rs` (one listener a port, owner only, bounded backlog; net.sockets alone, Direct only) | `userland/net_proofs` `net_listen` |
| net.core requests on `net.tcp`, and the LISTEN and ACCEPT bodies | `userland/capsule_net_core/src/server/parse_req.rs` `parse` and `server/handlers/tcp/passive_body.rs` `listen_port`, `accept_handle` (never past the bytes sent; port 0 refused) | `userland/net_core_proofs` `net_tcp_passive` |
| capsule manifest | `src/security/capsule_manifest/decode` | `userland/admission_proofs` `manifest` |
| NONOS-ID certificate | `src/security/nonos_id_cert/decode` | `userland/admission_proofs` `id_cert` |
| STARK attestation trailer | `nonos-attest-path` `parse_v4` (the crate the kernel's spawn gate calls, `src/security/capsule_attest/path.rs`); `stark-attest` `verify_attestation_trailer` | `nonos-attest-path` `v4_parse`; `stark-attest` `trailer` |
| `.approval` record (104-byte binary, not CBOR) | `nonos-boot-measure` `record::parse` | `nonos-boot-measure` `record_parse` (any bytes), `boot_record` (mutated real records through `check`) |
| Nym gateway and client bytes | `capsule_net_nym` ws frame, fragment, padding, request header, reply reassembly and reply message | `userland/nym_reply_proofs` `nym_inbound` |
| market catalogue and replies | `userland/market_proto` readers and `reply_body` | `userland/market_proto` `catalogue`, `linux_policy` |
| firmware AML (DSDT, SSDT) and the hardware behind it | `userland/nonos_aml` block, parser, loader, interpreter | `userland/nonos_aml` `aml` |
| capsule ELF header | `src/elf` | `userland/kernel_proofs` `elf_header` |
| web page markup | `capsule_browser` HTML input decode, tokenizer, tree builder and fragment parser | `userland/capsule_browser_html_proofs` `html` |
| Bluetooth controller events and ACL data | `nonos_bt_wire` HCI event and LE meta readers, ACL header, L2CAP reassembly, LE signaling, ATT responses | `userland/nonos_bt_wire_proofs` `bt_wire` |
| SMBIOS entry point and structure table | `nonos_smbios` entry, walk, structure, text, identity, chassis, quirk; the kernel's entry check and copy | `userland/nonos_smbios_proofs` `smbios` |
| USB mass storage device answers | `capsule_driver_usb_msc` descriptors, CSW, SCSI replies, request parser, xHCI replies; `nonos_uas` status | `userland/usb_msc_proofs` `usb_msc` |
| flattened device tree from firmware | `src/arch` fdt header, blocks, nodes, properties and every finder (memory, cpus, reserved memory, uart, gic, timer, pci, psci, smmu, framebuffer, chosen) | `userland/fdt_proofs` `fdt` |
| monitor EDID | `nonos_edid` base block, CTA and DisplayID extensions, walk and query; the loader's `gop/pick.rs` reader | `userland/edid_proofs` `edid` |
| UFS controller registers, UPIU responses and unit data | `capsule_driver_ufs` bring-up, REPORT LUNS, READ CAPACITY, sense, block read and write | `userland/ufs_proofs` `ufs_volume` |
| eMMC and SD host registers, card registers and EXT_CSD | `capsule_driver_sdhci` bring-up, CID, CSD, EXT_CSD, the read/write request head; the kernel's eMMC and SD host lists | `userland/emmc_proofs` `emmc_volume` |
| SOCKS5 client bytes and the proxy's request frames | `capsule_socks5` request decode (`ask`) and the connection handshake machine | `userland/capsule_socks5_proofs` `socks5_request` |
| NVGP frames between the compositor and a display driver | `nonos_gfx_wire` header, request, reply and output notice | `userland/gfx_wire_proofs` `gfx_request` |
| NVGP bodies | `nonos_gfx_wire` CAPS, SCANOUT, STATE, SURFACE, FLUSH, MODES, CURSOR, and the resize rule over them | `userland/gfx_wire_proofs` `gfx_body` |
| a client's request bodies to the compositor | `userland/compositor/src/requests` (OP_LAYER_CONTENT held word for word, scene submit and damage clipped to the canvas, every other request read) | `userland/gfx_wire_proofs` `compositor_requests` |
| images a client asks image_codec to decode | `capsule_image_codec` `decode_sized` over PNG, BMP, GIF, JPEG and raw LZ4 | `userland/image_codec_proofs` `image_decode` |
| image_codec request frames and held outputs | `capsule_image_codec` request parse, reply header, output table | `userland/image_codec_proofs` `image_request` |
| what a Linux guest writes to the SOCKS endpoint at 127.0.0.1:9050 | `capsule_linux` `linux/net/routed/socks` SOCKS5 greeting, RFC 1929 username and password, SOCKS4 and SOCKS4a requests and the conversation over them | `userland/capsule_linux_proofs` `linux_socks` |
| the 44 bytes a Linux guest hands TCSETS2, TCSETSW2 or TCSETSF2 | `capsule_linux` `linux/console/termios2.rs` `from2` (refused unless 44 bytes; only c_cflag's two line rates are chosen, each the code named or, by number, a named rate of that speed or the one there was; settings read back by TCGETS2 and set again come back unchanged) | `userland/capsule_linux_proofs` `termios2_set` |
| the bytes the NONOS terminal sends a Linux console, under any c_iflag a guest set | `capsule_linux` `linux/console/tty_rules.rs` `map_input` (each byte read is the one typed, in order, its eighth bit cleared under ISTRIP, a carriage return dropped under IGNCR or a newline under ICRNL, a newline a carriage return under INLCR, every other byte as typed) | `userland/capsule_linux_proofs` `tty_input_map` |
| the boxes a Linux guest names in wl_surface.damage_buffer, at any scale, and the box a commit shows | `capsule_linux` `linux/wayland/damage_boxes.rs` `Boxes` (at most four kept, none empty, each named box inside one kept box and every kept box inside the one box round all named; cut to the box shown, each lies inside it) | `userland/capsule_linux_proofs` `damage_boxes` |
| a Linux guest's Wayland requests, raw wire bytes, shm descriptors and desktop input | the Linux personality's Wayland server (`capsule_linux` `linux/wayland`) decode, dispatch, pools, surfaces, xdg roles, seat, data devices and teardown | `userland/capsule_wayland_proofs` `wayland_session` |
| a line typed in an nsh session | `userland/nonos_shell` `Session::run_line` (service, ping, nslookup and every other builtin; the host asked only for a registry name or a DNS name, and a ping's few echoes) | `userland/nsh_server_proofs` `nsh_shell_line` |
