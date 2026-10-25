
hw/pointer-range 447d85013 (on main 82a2a42b7) against main 82a2a42b7, each
built as standard-dev by `tools/nonos-dev-image`'s steps in its own copy of
the tree, sealed with throwaway keys; no key under `.keys/` was read.
QEMU only, under TCG. **Not hardware-verified.**

## The machine

`qemu.txt`: q35 with an Intel IOMMU, 8 GiB, 4 vCPUs, virtio-vga at
1920x1080, `usb-tablet` on the xHCI bus, no TPM; the firmware's GOP mode
was 1280x800. The kept data disk opens first-boot setup, so every move lands
on the boot splash and the setup screen.

## The probe

`earlyprobe.py` waits for the kernel's `first frame let through` line, then
every 3 s moves the pointer by QMP `input-send-event` (absolute, 0 to 0x7FFF
over the display) to (400,300) and (1600,900) in turn, takes a screendump,
and says where the cursor appeared against the frame before: at the target
(`exact`), where a 1024x768 range puts it (`default`, x * 1024 / 1920), or
neither.

## Three runs

| Run | Image | Compositor | Moves | Landed |
|---|---|---|---|---|
| unpatched | main + hw/dev-twin-serial d239e6c0d | as shipped | 49, 5 s to 150 s | 49 exact (`probe-unpatched.txt`, `unpatched.serial.log`) |
| before | main 82a2a42b7 | late-answer patch | 33, 5 s to 102 s | 33 at the 1024x768 default (`probe-before.txt`, `before.serial.log`) |
| after | 447d85013 | late-answer patch | 33, 5 s to 102 s | 33 exact (`probe-after.txt`, `after.serial.log`) |

On this machine the compositor answers the router's DISPLAY_INFO within the
router's 150 ms, so as shipped the bug does not show (the first run). It
needs a late answer, as build-and-boot's runs had. The before and after runs
force one with `scratch-late-answer.NEVER-APPLIED.patch`, a scratch-only
change to the compositor's DISPLAY_INFO handler that leaves the input
router's asks (and only the router's) unanswered for the first 150 s. **It
was never applied to any committed build, and is kept here only so the runs
can be repeated.** Both images carried it, so the fix is the only difference
between them.

Before, the router never learnt the size and placed every move on
CursorState::new()'s 1024x768: (1600,900) at (853,640), (400,300) at
(213,213). After, the compositor's NIRS 0x0006 notice set the edges and every
move landed where it was aimed. One frame of the after run (move 10) also
changed near the default spot, as the same frame of the unpatched run did:
the screen redrawing there, not the pointer.

`before-after.png`: move 2 of each run, to (1600,900), 2 s in; green is the
target, red where a 1024x768 range puts it.
