
Every machine without a native display driver in the image (all AMD and
NVIDIA graphics, and Intel until intel_display is sealed) shows NONOS on the
framebuffer the UEFI firmware set up. Booted only under OVMF in QEMU; no
hardware report names it. A firmware with no 32-bit linear mode is named

## What the image needs

A development image (`make dev-image`: the `dev` profile, or `<profile>-dev`),
built from main and signed with throwaway dev keys, commit recorded. Development
Standard; without it no driver's own lines, and no `log` copy of the
console, exist. The machine boots UEFI with CSM (legacy BIOS) off: NONOS
has no legacy boot path.

## Steps

1. Note the machine's panel or monitor resolution and the graphics in it
   (from Linux: `lspci -nn | grep -iE 'vga|display|3d'`, and
   `cat /sys/class/drm/*/modes | head -3` for the native mode).
2. Boot NONOS. Note whether a picture appears at each stage: the firmware
   logo, the NONOS splash, the desktop. If the screen goes black and stays
   black, wait two minutes, then note it.
3. On the desktop, Terminal: `display` (the size NONOS draws at) and
   `log gop display compositor framebuffer > gop-1.txt`.
4. With an external monitor attached, boot again with only the external
   one connected, then with both, and note which one shows the desktop.
5. Only on a machine where step 2 showed no NONOS splash (a CSM-only
   graphics card, or firmware whose display is PixelBltOnly or 16-bit):
   with a serial console attached, boot again and keep the whole serial
   log, at least two minutes of it. Without a serial console there is no
   way to read the reason on that machine; say so in the report.

## What passes

- A desktop at the panel's native size, or at the firmware's mode where
  EDID was not readable; text sharp, colours right (red is red: a swap of
  red and blue is a finding).
- `display` agrees with what step 1 says the panel is, or the line in
  `gop-1.txt` says why not.
- Step 5: the loader's `[GOP] no linear framebuffer: <why>`, the kernel's
  `[GOP] no framebuffer handed over: <why>`, the compositor's `[DISPLAY] no
  display yet (no framebuffer: <why>)` once, and 10 s on `[DISPLAY] still no
  display 10 s on (...)` once. The three name the same reason. That is the
  named failure working, not a pass for the display: the machine still has
  no desktop until a display driver takes it.

## What to send

The commit, the CPU and graphics, the panel's native mode, what each stage
of step 2 showed, the `display` output, `gop-1.txt`, and for step 4 which
screen showed the desktop. A black screen with a serial console: send the
serial log; without one, the stage it stopped at.
