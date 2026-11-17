# Colours

Where every colour an app draws comes from: five seeds per theme, the 26 [roles](../../overview/glossary.md#role) worked out from them, and the contrast floors the host proofs hold each role to.

## Seeds

A theme starts as a `Scheme` of five seeds, `bg`, `surface`, `accent`, `text` and `border` (`userland/toolkit/src/theme/schemes/table.rs:24-30`, `Scheme`). There are nine, in `THEME_LABELS` order: Aurora, Slate, Nord, Dracula, Solar, Mono, Forest, Sunset and Light (`userland/toolkit/src/theme/schemes/table.rs:38-48`, `SCHEMES`; `userland/policy_proto/src/theme_labels.rs:17-18`). Aurora is index 0 and the default, and an index out of range falls back to it (`userland/toolkit/src/theme/schemes/table.rs:53-58`, `scheme`).

High contrast is not a tenth entry. `theme_of(index, high_contrast)` takes `HIGH_CONTRAST` in place of the chosen scheme when the flag is set (`userland/toolkit/src/theme/select.rs:24-27`, `theme_of`): black ground and surface, a yellow accent, white text and border (`userland/toolkit/src/theme/schemes/contrast.rs:38-44`, `HIGH_CONTRAST`).

## Roles

A widget draws with roles, never with a seed or a literal. `theme::roles()` returns the `Roles` value for the theme in force, cached and worked out again only when the stored theme changes (`userland/toolkit/src/theme/roles/cache.rs:31-42`, `roles`). `from_theme` works each role out from the seeds with `derive::mix`, where `t` runs out of 255 (`userland/toolkit/src/theme/roles/from_theme.rs:34`; `userland/toolkit/src/theme/derive.rs:33`, `mix`).

The **state grounds** are the four flat grounds, `bg`, `sunken`, `surface` and `raised`, each also under `accent_wash`, under `hover_wash` and under both: sixteen in all (`userland/toolkit/src/theme/roles/grounds.rs:34-56`, `flat_grounds`, `state_grounds`). A widget may draw ink on any of them, so every ink below is held to its floor on all sixteen.

| Role | Type | Used for | Worked out as |
|---|---|---|---|
| `bg` | opaque | window ground | seed |
| `sunken` | opaque | sidebar, header, status strip | `mix(bg, black, 12)` |
| `surface` | opaque | card, pane | seed |
| `inset` | opaque | table body, list inside a card | `mix(bg, surface, 128)` |
| `raised` | opaque | field, pill, selected card | `mix(surface, accent, 11)` |
| `border` | opaque | hairline | seed |
| `border_soft` | opaque | a line inside one surface | `mix(bg, border, 87)` |
| `border_strong` | opaque | control outline | `mix(border, text, 16)` |
| `text` | opaque | body and titles | seed |
| `text_2` | opaque | a value beside its label | `mix(text, bg, 36)`, stepped back to clear 4.5:1 |
| `muted` | opaque | secondary text | `mix(text, bg, 86)`, stepped back the same way |
| `quiet` | opaque | placeholder, hint | `mix(text, bg, 128)`, stepped back to clear 3:1 |
| `disabled` | opaque | an unavailable control | `mix(border, text, 57)` |
| `accent` | opaque | links, on states, the brand | seed |
| `accent_text` | opaque | an active tab or nav label | `mix(text, accent, 109)`, stepped back towards `text` to clear 4.5:1 |
| `on_accent` | opaque | ink on an accent fill | the first of `bg`, `text`, black, white to reach 4.5:1 |
| `accent_wash` | translucent | selected or active fill | `accent` at 0x20, thinned until `text` reads through it |
| `accent_edge` | translucent | active outline | `with_alpha(accent, 0x59)` |
| `focus_ring` | translucent | keyboard focus | `with_alpha(accent, 0x80)` |
| `hover_wash` | translucent | hovered row | `text` at 0x14, thinned the same way |
| `ok`, `warn`, `danger` | opaque | status | the hue from `theme/palette.rs`, lifted to clear 4.5:1 |
| `warn_soft` | opaque | warning banner | fixed: dark `0xFF3A2A0C`, light `0xFFFEF3C7` |
| `shadow` | translucent | drop shadow | fixed: dark `0x60000000`, light `0x30101828` |
| `scrim` | translucent | backing behind a label on the wallpaper | fixed: `0x8C060A0E` |

The targets 36, 86, 128 and 109 are `VALUE_TARGET`, `MUTED_TARGET`, `QUIET_TARGET` and `ACCENT_TEXT_TARGET` (`userland/toolkit/src/theme/legible.rs:25-35`). The stepping back is `walk_on` (`userland/toolkit/src/theme/legible_walk.rs:44`), the wash thinning is `thin` (`userland/toolkit/src/theme/wash_walk.rs:42`), the status lift is `lift` (`userland/toolkit/src/theme/lift.rs:56`) and the fixed values are `fixed` (`userland/toolkit/src/theme/roles/fixed.rs:39`). A colour only one widget needs, such as a switch's knob, is worked out inside that widget.

## Colour types

A role is an `Opaque` or a `Translucent`, two wrappers round an ARGB word that both give it back with `argb()` (`userland/toolkit/src/theme/colour.rs:22-47`). The widgets' typed painters keep them apart: `draw::fill` writes without blending and takes only an `Opaque`, while `draw::blend`, `round` and `outline` take either (`userland/toolkit/src/widgets/draw/painters.rs:27-42`). A translucent role therefore cannot reach a raw write, which is how the desktop shell once punched a hole through its own dock tiles.

## Readability

`theme_proofs` checks every scheme, Light and high contrast included:

- 4.5:1 for `text`, `text_2`, `muted`, `accent_text`, `ok`, `warn` and `danger` on every state ground, and for `on_accent` on `accent` (`userland/theme_proofs/src/roles_tests.rs:47`, `body_inks_hold_the_body_floor`).
- 3:1 for `quiet` on every state ground (`userland/theme_proofs/src/roles_tests.rs:67`, `quiet_inks_hold_the_large_floor`).
- `disabled`, the borders and the washes carry no floor, since none is read as text.

The floors are `BODY` (45 tenths) and `LARGE` (30 tenths) (`userland/toolkit/src/theme/contrast.rs:47-51`). The `on_accent` rule exists for Solarized, whose `bg` reads at only 4.08:1 on its accent (`userland/toolkit/src/theme/roles/on_accent.rs:23`). The stepping back exists for it too: its text reads at only 4.86:1 on its own surface, so a fixed step towards the ground would fail, and its hover wash thins to nothing.

A status ink is lifted, not mixed towards `text`: its channels are scaled by one factor, which keeps the hue, and only once a channel reaches 255 (or black, in Light) is it mixed towards white (or black). Mixing towards a cool scheme's text turned Nord's warning grey. Red carries little luminance of its own, so on Nord's and Dracula's lighter washed grounds `danger` ends a pale red.

## What colour does not cover

- A theme the person designs. The nine schemes and high contrast are what is offered.
- The wallpaper, which is a picture on its own compositor layer, not a role.
- A web page in the Browser, which draws with the page's own CSS.
