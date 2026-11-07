# capsule_input_router: Routing ahead of focus

Part of [capsule_input_router](README.md).

## Routing ahead of focus

- Ahead of grabs and focus (`route/preempt.rs`): the press of a reserved key
  goes to the shell (Ctrl+Alt+Esc; Ctrl+Alt+Delete, the secure attention
  chord; the system keys), its release follows it, and while the compositor's
  trusted band is held (a lock screen or a permission prompt is up) every key,
  click, motion and wheel goes to its holder alone, and a grab requested by
  anyone else is refused with E_BUSY.
- Bluetooth pairing (G-76, `route/bond.rs`, `route/refuse.rs`): the Bluetooth
  capsule stamps bit 15 of an event's flags (`SOURCE_BT`) on every event it
  posts, and bit 14 (`BOND_AUTHENTICATED`) when the device's bond was
  authenticated; the kernel keeps both, writing its source index into bits 8
  to 13 only (K8b). An event with bit 15 and not bit 14 (a device paired by
  Just Works, or one not stamped with its level) never reaches the secure
  path: the secure attention chord, press and release, goes nowhere, and while
  the trusted band is held its keys, clicks and motion go nowhere either,
  except a release going back to the window its press went to. Its other
  events route as any other. The shell is told at most once a minute
  (`route/notice.rs`). Events without bit 15 come from wired devices and are
  untouched. Bits 8 to 15 are cleared from every delivery.
- Gamepads (B-57: kinds 8 button down, 9 button up, 10 axis; the pad's number
  in y) follow focus exactly as keys do (`route/pad.rs`): a press and an axis
  go to the focused window, a release to where its press went, and no other
  subscriber hears them.

## Notices

A delivery header's flags are 0, except the router's notice to the shell that
an unauthenticated Bluetooth device was refused (`NOTICE_UNPROVEN`, 1), whose
event is kind `NOTICE_KIND` (0xFFFF) and otherwise zero.
