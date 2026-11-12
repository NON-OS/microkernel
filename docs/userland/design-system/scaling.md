# Display scale, turned on

[Sizes](sizes.md) defines the scale: quarters, with 4 for 1×, and `scale().px`, `hair` and
`font`. Version 1 kept it at 4 everywhere. This page is how it gets turned on.

## Today

- The skeleton sets the toolkit's scale when it learns the display, to the quarters it draws
  the window frame at (`app_skeleton/src/runner/chrome.rs`, `widget_quarters`). A 1× panel
  stays one to one.
- Every app on the skeleton keeps logical sizes and draws them through `scale().px` or
  `scale().font`; the installer and the wallet keep their own scale (below).
- `fit_display` multiplies the manifest's content size by the scale before it adds the frame
  and caps to the work area, so a manifest's size is logical.
- The browser lays pages out in CSS px against the pane over the scale and carries the laid-out
  page into device px once, so a page at 2× draws at 2× (`layout/boxmodel/post/device.rs`).
- Proven on the host at 4, 5, 6 and 8 quarters per app; not yet booted at 2×.

## The design

1. **One scale per process.** When the skeleton fits the window to the display, it calls
   `set_scale(quarters)`, in the same place it stores `QUARTERS`. Widgets already convert
   through it.
2. **Sizes become logical.** Each app's `metrics.rs` keeps logical numbers. Every use goes
   through `scale().px` or `scale().font`, in the one geometry function that its painter and its
   hit test share. A metric that only one shared widget used goes away with that widget.
3. **The content area scales too.** `fit_display` multiplies the manifest's content size by the
   scale, capped by `WINDOW_FRACTION` as today. A manifest's size is logical from then on.
4. **One switch.** The step that passes the real quarters is its own commit (004047250), so a
   boot that shows a problem reverts it alone.

## What stays

- The boot splash, the installer and the wallet keep their own scale, through
  `nonos_brand::scale`.
- The rule that picks quarters from the panel size stays in `scale_rule::quarters_for`. Its copy
  in `capsule_install/brand/src/scale_rule.rs` is noted, not merged, since the installer runs
  outside the desktop.

## Tests

- Each app's proofs crate, where it has one: its geometry at 4, 5, 6 and 8 quarters stays inside
  the window, and draw and hit still agree.
- The widget proofs already draw at 1×, 1.25× and 2×.
- The switch to real quarters is believed only after a boot, on a 1× panel and on a large one.

## Open

- ✅ Units at the widget boundary, settled by the owner on 2026-11-11: rects, points, sizes and
  widths crossing the boundary are drawing pixels; a size an app declares as a setting
  (`Col::fixed`, `icon_size`) is logical and the widget scales it. `widgets.md` states the rule.
  This matches the code, so no widget changed.

- ❓ Whether the compositor's half-size canvas for large panels
  (`capsule_desktop_shell/src/state/scale.rs`) should change once apps scale, or stay. This is
  measured on a boot.
