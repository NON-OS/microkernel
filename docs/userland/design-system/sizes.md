# Sizes

The spacing, radii, heights and text sizes every widget is built from, all in logical pixels: one logical pixel is one screen pixel at a [display scale](../../overview/glossary.md#display-scale) of 1x, and [Display scale](scaling.md) turns them into screen pixels.

## Where they live

`nonos_toolkit::design` holds four modules: `scale`, `size`, `spacing` and `text_role` (`userland/toolkit/src/design/mod.rs:1-4`). `size.rs` has every constant below, `text_role.rs` the text sizes, and `scale.rs` the conversion. The widgets read them, and so do eleven capsules that draw a few shapes of their own, from the Audio Player to the Video Player (`grep -rl design::size userland/*/src`).

## Spacing

`SPACE` is `xs` 4, `sm` 8, `md` 12, `lg` 16 and `xl` 24 (`userland/toolkit/src/design/size.rs:22`, `SPACE`), a `SpacingScale` from `spacing.rs`, which also gives `Insets` with `uniform` and `hv` constructors (`userland/toolkit/src/design/spacing.rs:1-32`).

## Radii

| Name | Value | Used by |
|---|---|---|
| `CONTROL_RADIUS` | 8 | buttons, fields, menus (`userland/toolkit/src/design/size.rs:24`) |
| `CARD_RADIUS` | 12 | cards, dialogs (`userland/toolkit/src/design/size.rs:27`) |
| `CHECK_RADIUS` | 5 | the checkbox box (`userland/toolkit/src/design/size.rs:93`) |
| half the height | | pills, switches, the search field (`userland/toolkit/src/design/size.rs:25-26`) |

## Heights and widths

| Widget | Constant | Logical px |
|---|---|---|
| row | `ROW_H`, `ROW_NOTE_H` | 44, or 66 with a note |
| card head | `CARD_HEAD_H`, `CARD_HEAD_NOTE_H` | 48, or 74 with a note |
| nav item | `NAV_H` | 38 |
| button, field | `CONTROL_H` | 36 |
| field | `FIELD_W` | 236 wide |
| tabs | `TABS_H` | 32 |
| pill | `PILL_H` | 26 |
| menu row, table row | `MENU_ROW_H`, `TABLE_ROW_H` | 32 |
| switch | `SWITCH_W`, `SWITCH_H` | 42 by 24 |
| checkbox | `CHECK` | 20 |
| slider | `SLIDER_W`, `TRACK_H`, `KNOB_R` | 150 wide, track 4, knob radius 7 |
| progress | `PROGRESS_H` | 6 |
| scrollbar | `SCROLL_W`, `SCROLL_MIN` | 4 wide, thumb at least 18 |
| dialog, toast | `DIALOG_W`, `TOAST_W` | 440 and 360 wide |
| icon | `ICON` | 18 |

All are in `userland/toolkit/src/design/size.rs:29-109`.

## Text

| Role | Logical px | For |
|---|---|---|
| `TextRole::Body` | 17 | ordinary text |
| `TextRole::Title` | 21 | the title of a page or card |
| `TextRole::Headline` | 26 | the largest heading on a screen |

The sizes are `TextRole::px` (`userland/toolkit/src/design/text_role.rs:21-40`). A widget takes `scale().font(TextRole::Body.px())` (`userland/toolkit/src/widgets/button/geometry.rs:46-48`), and the text layer then passes it through `readable_px`, which raises anything under `MIN_UI_PX`, 17.0, and applies the person's text-size setting (`userland/toolkit/src/font/ttf/readable.rs:22-37`). That setting has five steps, Tiny to Huge (`userland/toolkit/src/font/ttf/text_scale.rs:26`). Line boxes come from `line_height`, widths from `measure` (`userland/toolkit/src/font/ttf/metrics.rs:26-59`). Nothing is sized by counting characters, since the fonts are proportional.

## What sizes do not do

- They are not screen pixels. A constant used raw draws at 1x on every display; it goes through `scale().px` first.
- They are not a layout engine. A widget is drawn into a rect the app works out, with the helpers in `widgets::rect` and `Column`.
