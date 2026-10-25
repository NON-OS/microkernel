# Worker reply: a service's answer reaches a thread's call, booted

One scratch boot of privacy/worker-reply at **2ebfb9396**, which is 089c5792c merged with main 0d982b49f, S-13b included (`head.txt`).

## The image

- **Built with:** `tools/nonos-dev-image` with throwaway keys, image sha256 `5b8054c4…d32f9`.
- **Profile `wrproof`, scratch tree only** (`boot.sh.txt`): the kernel features `microkernel-full-gui` and `nonos-capsule-proof-io`, with nothing dropped.
- **The prober:** capsule_proof_io's `main.rs` is `proof_main.rs.txt`, holding IPC and Memory only (`caps=0x118`). It makes two calls, first from its main thread, then from a `WorkerSeat` thread:
  - the installer: remove "zz";
  - crypto_pool: a request it cannot decode.

  Both services answer through the kernel's reply redirect. Each call must be answered, with any status, within 10 s. The exit code is 42 when all four were, 2x when the main thread's call x was not, and 3x when the worker's was not.
- **QEMU:** 11.1.1, TCG, `-smp 4`, 6 GiB, `--tpm`, lanes `iommu=intel`, `usb-kbd` and `usb-tablet`. Setup was walked with the D-13 keys.

## What the serial log shows (`serial.log`)

- **The prober:** `[EXIT] code=42 proof_io` (line 1229). The installer and crypto_pool each answered the worker thread as well as the main thread.
- **The Terminal's `pkg` jobs,** each run on its worker thread and admitted under the Terminal's mask by S-13b's gate (`caps=0x…C100197D`, line 1264). Each was answered within the 5 s screenshot:

  | Command | Time | Answer | Screenshot |
  |---|---|---|---|
  | `pkg remove gui_demo` | 45 ms | `removed gui_demo` | `30-5s-pkg_remove_gui_demo.png` |
  | `pkg install /home/nonos/readme.txt` | 38 ms | `pkg: malformed package or path` | `31-5s-…readme.txt.png` |
  | `pkg remove zz` | 66 ms | `pkg: not installed` | `32-5s-pkg_remove_zz.png` |

  On main before the fix (282a5eed4), each of these ran its op but timed out after 29.8 s with `[DIAG] ipc.call unanswered installer err=-110`.
- **Lines that are absent:** no `unanswered` line, no `reply refused` line, and no FATAL, PANIC, #PF, #GP, `panicked` or ZEROSTATE line.
- **What is present:** seven `ipc.call late` lines, for virtio_net0, wallpaper, net.dhcp.client and toolkit. These are callers' own short deadlines running out, and none involves a thread or a redirect service.
- **Not shown in this boot:**
  - A refused answer can't easily be forced here. The rule is proven on the host: kernel_proofs `syscall::microkernel::reply_dest`, 7 tests, with 13 of 13 mutants caught (`mutants.txt`).
  - nonos_tls's RSA checks on the audio player's and wallpaper's fetch workers need an HTTPS peer with an RSA chain. That case is reasoned from the code: it is the same crypto_pool call from a thread that the prober makes.
- **How it ended:** QEMU was stopped through QMP `quit` at 19:31:32 UTC, after the last screenshot.
- **An earlier attempt at the same head never booted:** `data.img` was write-locked by a QEMU left running from an earlier boot (`Failed to get "write" lock`). That QEMU had exited by the time of this run.
