# capsule_driver_ps2_input: privacy and ownership

What the capsule keeps of the input it handles, what it owns, and the rules
it keeps. The summary is in [README.md](README.md).

## Privacy and persistence

Keyboard and pointer input is treated as private live data. The capsule keeps no
key once it is posted (only the code of a key still held, which its release
carries), a mouse queue emptied on every pass, and counters. It does not write
input to disk, forward input to unrelated services, keep history after process
exit, or decide where input should be delivered. Focus, compositor routing,
lock-screen policy, accessibility policy, compose keys, and text rendering
belong to higher-level userland capsules. The keyboard layout is the one
exception: nothing downstream resolves shift or layout, so the driver translates
each key through the active layout (`nonos_keymap`), reads the choice from the
policy store at most once a second, and cycles it on Ctrl+Alt+Space
(`src/keymap/active.rs`).

## State ownership and operating rules

The capsule owns the i8042 PIO grant, IRQ1 grant, IRQ12 grant, keyboard decoder
state (prefixes, modifiers, lock state, held keys), the active layout, AUX
packet parser state, the mouse queue, and when the mouse was last brought up
again. It does not own focus, routing, cursor policy, compositor state, or input
persistence. Those decisions are made by higher-level userland capsules.

Keep all i8042 access behind `MkPioRead` and `MkPioWrite`; keep IRQ ownership
explicit, keyboard on IRQ1 and AUX mouse on IRQ12; never persist keyboard or
pointer input; never route input to windows or make focus or lock-screen
decisions here; keep no keystroke once it is posted; drop deterministically on
mouse-queue overflow and expose the counter; treat packet sync loss as
data-plane damage, not as a kernel fault.

## See also

- [README.md](README.md)
- [CONTRACT.md](CONTRACT.md)
