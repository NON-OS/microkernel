# The NØNOS interface identity

Version 0.2, 2026-10-25. v0.1 was approved by the Lead as the standard every
app unit is checked against, with seven changes, which v0.2 makes. Items marked
**proposed, owner to confirm** are built as one token or setting each, so the
owner's answer is a data change.
Nothing here is code yet. Every app unit after this one is checked against
this page, and a screendump that breaks a rule here does not land.

## Where it comes from

This page does not invent a brand. Three sources already exist:

- **The site's brand page, nonos.software/brand.** It is the authority for
  the mark, the colours, the type and the voice. Its kit is the SVGs under
  `/assets/brand/nonos-brand-kit.zip`.
- **`userland/capsule_install/brand` (`nonos_brand`).** The installer and
  first-boot setup draw with it. It has the site's colours, Geist and
  JetBrains Mono, an 8 px unit, a 1.2 type ramp and the canvas scale rule.
  It is the only part of the desktop that already looks like NØNOS.
- **The toolkit's theme engine, `userland/toolkit/src/theme`.** Five seed
  colours derive every chrome role, and `theme_proofs` checks contrast. It
  is the right machine, but it is loaded with the wrong colours: a teal
  "Aurora" palette and seven others that are not ours.

The work is to load the site's identity into the toolkit's machine, so that
there is one system. `nonos_brand` becomes a thin re-export of the toolkit
and every app reads from the toolkit. No second system is started (rule 6).

Where the site is silent (the type scale, the grid, icons, interface motion,
window chrome), this page decides. It marks each such decision **proposed**
so the Lead and the owner can rule on it.

## Principles

These five decide trade-offs. When two rules conflict, the earlier one wins.

1. **Show the proof, not the polish.** Anything a person might verify (a
   hash, a version, an amount, a capability, a count) is shown in full, in
   mono, with a way to copy it. Decoration that proves nothing is removed.
   This is the site's rule, "prefer a number to an adjective", applied to
   pixels.
2. **Quiet until it matters.** The ground is ink and the text is paper.
   Cyan is spent on one thing per view: the current place or the one action
   that moves you forward. Red is spent only on loss and failure. If
   everything is highlighted, nothing is.
3. **Dense where you read, open where you decide.** Lists, tables and the
   Terminal are tight, on a 4 px half-step. Screens that ask for consent,
   money or a disk get the open spacing of the site, 32 to 64 px, with one
   question each.
4. **Every state is designed.** Empty, loading, error, full, long text,
   narrow window, focused, disabled. A state nobody drew is a bug, not an
   edge case.
5. **The keyboard is not second.** Everything the pointer can do, keys can
   do. Focus is always visible, and Shift+Tab always goes back.

## The mark

These rules come from the site.
- **Shape.** The default is a square of 450 by 500 with a corner of 80 and
  the Ø cut out of it. A circle is for badges. The Ø alone is for very small
  spaces.
- **Clear space.** A quarter of the mark's height on every side.
- **Minimum size.** 24 px for the mark and 120 px wide for the full logo.
  Below that, use the mark alone.

Placement in the OS (**proposed**):

| Where | Version | Size at 1x |
|---|---|---|
| Menu bar, left end | Ø alone, text colour (not cyan) | 16 px, the one place below 24, because the site allows the Ø alone in very small spaces |
| About, header | Square mark, cyan | 64 px |
| Boot splash, setup, installer | Square mark, cyan, **with its glow** | the installer's 56, 96 or 150 |
| Lock screen | Square mark, white | 48 px |

- **Where it never goes:** inside app windows other than About, on buttons,
  as a loading spinner, rotated, outlined, or over a busy wallpaper in
  colour. The site says to use white there.
- **Owner ruling, 2026-10-25:** the glow on the lit mark in the installer is
  wanted and stays. It is the one exception to the site's "no glows". Asked
  of the owner and not yet answered: whether setup keeps it too. Nowhere
  else gets it without a ruling.

## Grid and spacing

**Proposed.** It extends `nonos_brand::scale` to the whole desktop.

- **Unit.** One unit is 8 logical px, with a half-step of 4 px for dense
  rows.
- **Scale.** `space.1` to `space.8` are 4, 8, 12, 16, 24, 32, 48 and 64.
  No other spacing number appears in app code. A static check, the third
  unit, can find literals.
- **Logical pixels.** Every size on this page is logical. The canvas scale
  is the existing rule (`scale_rule.rs`): 1x below a 1000 px short side,
  1.25x from 1000, 1.5x from 1440 and 2x from 2160. It is applied once, by
  the toolkit, never by an app.
- **Row heights.**

  | Row | Height |
  |---|---|
  | Dense list or table | 28 (`space.4` text box + 2 × 6) |
  | Standard list | 36 |
  | Settings row | 48 |
  | Touch-safe control | at least 32, hit area at least 24 everywhere |

  Rows are sized from the text's line height, never a fixed number. Large
  and Huge text then grow the rows instead of clipping them, which About
  and Processes do today.
- **Line length.** Body copy wraps at 72 characters at most. Consent and
  error text wraps at 60.

## Type

The site picks the faces. Both are SIL OFL, already in the tree with their
licences.

| Role | Face | Weight | Use |
|---|---|---|---|
| Display | Geist | 500 (Medium) | window and screen titles, the one headline per view |
| Text | Geist | 400 (Regular) | everything people read |
| Mono | JetBrains Mono | 400 | hashes, addresses, versions, amounts, sizes, pids, paths, the Terminal, small capitals labels |

**Proposed:**
- **Drop the other system faces.** Noto Sans stays only as the fallback for
  scripts Geist lacks (Arabic, Devanagari, Hebrew, the wider Latin). Inter
  leaves the system UI. The Editor may keep it as a document font.
- **Scripts, as they are.** Noto draws Arabic, Hebrew and Devanagari glyphs,
  but the toolkit lays every line out left to right and has no bidi: Arabic
  and Hebrew text comes out in the wrong order, and Devanagari without
  reordering of its vowel signs. A bidi and shaping unit is on the list after
  the apps.
- **Geist 600** (proposed, owner to confirm). The site asks for 600 beside the mark. That needs
  Geist-SemiBold added with its licence. It is used only for the wordmark
  and the menu bar's app name.
- **The scale.** This is the brand's 1.2 ramp, already proven by
  setup_layout_proofs, now for every app:

  | Token | px at 1x | Line | Face | Use |
  |---|---|---|---|---|
  | `type.label` | 12 | 16 | Mono, capitals, +8% tracking | section labels, column heads, key hints |
  | `type.caption` | 14 | 20 | Geist 400 | secondary lines, metadata |
  | `type.body` | 17 | 24 | Geist 400 | default text |
  | `type.heading` | 20 | 28 | Geist 500 | card and section titles |
  | `type.lead` | 24 | 32 | Geist 500 | one-line summary of a screen |
  | `type.title` | 35 | 40 | Geist 500, -1% tracking | the title of a full-screen step only |
  | `type.data` | 14 or 17 | as text | Mono, tabular | numbers in columns |
- **Numbers.** Any column of numbers uses tabular figures: the mono face, or
  Geist's `tnum` where it is shaped. Amounts align on the decimal point.
- **Casing.** Sentence case everywhere. Capitals only in `type.label`, and
  in the name NØNOS. Footer hints like `ENTER NEXT  ESC BACK` become labels
  with the keys in mono boxes: `Enter` Next.
- **The 17 px floor** (proposed, owner to confirm). `ttf::MIN_UI_PX = 17`
  lifts every size below 17 to 17 today. That is why every app's hierarchy
  is flat. The roles above are built (`design::typography`, proven in
  theme_proofs' design_token_tests). The floor itself goes with the shell
  unit, which sizes its UI from it. It is replaced by:
  - nothing is drawn below 12;
  - only `type.label` (mono capitals, high contrast) is drawn below 14;
  - the user's text-size setting scales every role together.
  
  theme_proofs gets a proof that no role resolves below its floor at any
  text scale, and a mutant that lowers one.

## Colour

Values come from the site, and roles from this page. All of them live in one
place, `toolkit/src/design/roles_set.rs`. The theme schemes take their seeds
from there. Every ratio below is asserted from those values in theme_proofs
(`roles_tests`), with a mutant for each group in
`verification/host_mutants/ui-roles.json`:
- text and text.2 at 7:1 on ground, surface and raised;
- text.3 at 4.5:1;
- edge at 3:1;
- the accent at 4.5:1 on ground and surface;
- on.accent at 4.5:1 on the accent;
- ok, warn and bad at 4.5:1 on every ground;
- on.bad at 4.5:1;
- text at 7:1 and text.2 at 4.5:1 on select;
- High contrast: everything at 7:1.

The figures in the tables are rounded down from the measured ratios.

### Ink (dark, the default)

| Role | Value | Notes |
|---|---|---|
| `ground` | `#0A0B0D` Ink | window and desktop ground |
| `surface` | `#101215` | cards, sidebars, menus |
| `raised` | `#16181B` | column heads, toolbars on a card |
| `text` | `#F2F3F7` Paper | 17.7:1 on ground |
| `text.2` | `#B3B4B7` | 8.5:1 on raised, the lowest of its three grounds; also the secondary line of a selected row (6.8:1 on select) |
| `text.3` | `#8A8B8F` | 5.2:1 on raised, metadata and placeholders, never on select |
| `rule` | `#26282B` | dividing lines, decoration only: under 3:1 by design |
| `edge` | `#62646A` | the outline of an input, switch or button, 3.0:1 on raised (WCAG 1.4.11) |
| `accent` | `#66FFFF` NØNOS Cyan | 15.4:1 on surface: the current place, the primary action, focus |
| `on.accent` | `#0A0B0D` | text on a cyan fill, 16.2:1 |
| `select` | `#12302F` | selected row fill, always beside a 2 px accent edge |
| `ok` | `#5EE0A0` | 10.7:1 on raised. Proposed, owner to confirm: the site names no success colour |
| `warn` | `#FFB547` | the installer's value, 10.1:1 on raised |
| `bad` | `#FF6B6F` | 6.4:1 on raised, with `on.bad` ink on a fill (7.1:1) |

### Paper (light)

The site's own ground is Paper, so a light scheme is ours and not a
borrowing.

| Role | Value | Notes |
|---|---|---|
| `ground` | `#F2F3F7` Paper | |
| `surface` | `#FFFFFF` | |
| `raised` | `#E8E9EE` | |
| `text` | `#0A0B0D` | 16.2:1 on raised |
| `text.2` | `#45474D` | 7.6:1 on raised |
| `text.3` | `#63656B` | 4.8:1 on raised |
| `rule` | `#D5D7DD` | decoration only |
| `edge` | `#7E8188` | 3.2:1 on raised |
| `accent` | `#007C82` | 4.5:1 on Paper, 4.99:1 on white. The site gives Cyan for type as `#007C85` at 4.5:1. It measures 4.49:1 on Paper, under AA, so this is the nearest value that clears it, three steps of blue apart. The site's cyan is never text on Paper (1.1:1), only fill. On raised the accent is not text. |
| `on.accent` | `#FFFFFF` | 4.99:1 |
| `select` | `#DCEFF0` | text 16.5:1 on it |
| `ok` | `#17703F` | 5.0:1 on raised. Proposed, owner to confirm |
| `warn` | `#8A5A00` | 4.8:1 on raised |
| `bad` | `#C4262E` | 4.7:1 on raised; white `on.bad` 5.7:1 |

### High contrast

This is the engine's existing HIGH_CONTRAST scheme: black ground, white
text and edges, and a yellow accent, which stays distinct from white under
the common colour blindnesses where a cyan does not. It is kept, and its
roles are in `roles_set.rs` beside Ink and Paper:
- text.3 is `#E6E6E6`;
- ok is `#66FF99`, warn `#FFD24D` and bad `#FF8A8A`;
- every one is at 7:1 or more on ground, surface and raised (`#141414`).

The focus ring is 3 px instead of 2, and hover washes become outlines.

### Balance

The site's balance (ink 26, paper 58, cyan 10, teal 4) is for printed
pieces. On a screen the rule is measured: `tools/nonos-ui-measure` prints
each screendump's accent share. That is the pixels at either accent, over
all pixels, wallpaper included. Every unit's report carries it.
- **Under 2% on any screendump.** The four wallpapers' desktops measure 0.07
  to 0.76%. The Wallet's welcome screen measures 5.43% today, and the
  Wallet unit brings it under.
- **Teal** only in selection.
- **No gradient fills** anywhere, the wallpaper excepted.

### Schemes

- **Built:** Ink (index 0, in place of Aurora, the default), Paper (index
  8, appended), and High contrast (the separate switch).
- **Unchanged:** the engine still carries Slate, Nord, Dracula, Solarized,
  Mono, Forest and Sunset as five-seed entries. Removing any of them is a
  data change, made once the owner rules.
- **Stored indices.** No stored index moved: Aurora's slot became Ink, and
  Paper was added at the end. So no migration is needed until a scheme is
  removed. That removal would map old indices to Ink in a versioned policy
  field, proven in policy_proofs with a mutant.
- **Owner to confirm:** whether Nord and Solarized stay as user options.

## Iconography

**Proposed.**
- **Today.** 48 masks in `userland/assets/icons`, drawn on a 20 grid. There
  are two stroke weights (1.5 and 1.667, the latter Lucide-like, in all
  Settings and Processes icons) and no recorded provenance. The masks are
  rasterized once at 192 px and scaled down at draw time, which blurs them
  at 16 to 24 px.
- **One set, drawn for NØNOS.**
  - A 20-unit grid with a 2-unit live-area margin.
  - A 1.5 stroke with round caps and joins, and no fills except the
    filled-state variant of a toggleable icon.
  - Outer corners follow the mark's proportion: its corner is 80 on a width
    of 450, about 1/5.6. On a 16-unit live area that gives a 3-unit radius.
    Inner corners are 1.5.
- **Rasterized per size.** The build script makes 16, 20, 24, 32 and 48 px
  masks, snapped to the pixel grid at each, not one 192 px master.
- **App icons.** Dock and Launchpad tiles are the same line glyphs on a
  `surface` square with the mark's corner proportion. There are no
  per-app colours. The running indicator is a 4 px cyan bar under the tile,
  not a dot.
- **Provenance.** Every SVG carries a header naming its author and the
  licence (AGPL, ours). `assets/icons/PROVENANCE.txt` lists every file, the
  way `wallpapers/MANIFEST.txt` does.
- **Never:** emoji, sparkles, mixed sets, or a coloured icon for meaning
  without a word beside it.

## Motion

The site gives the spirit: things resolve, draw in and settle, with no
bounce, spin or motion for its own sake. **Proposed** values:

| Token | Duration | Use |
|---|---|---|
| `motion.quick` | 90 ms | hover, press, focus ring |
| `motion.settle` | 180 ms | menus, popovers, toasts, the dock's launch mark |
| `motion.move` | 260 ms | window open, minimize and restore, the Launchpad |

- **Easing.** One family. `settle` is an ease-out cubic for anything
  arriving. `leave` is ease-in at two thirds of the duration for anything
  going. No springs or overshoot. The toolkit's existing `Curve::EaseOut`
  is quadratic, so a cubic is added beside it and the apps use the cubic.
- **What moves.**
  - A window scales from 98% and fades in.
  - The Launchpad fades and its grid settles 8 px upward.
  - A toast slides 8 px from the dock.
  - A progress bar draws itself as a line.
- **Character-by-character resolve.** The site's title resolve appears only
  on the boot splash and the first setup screen.
- **Never:** spinners that rotate (busy is a line that draws and travels),
  bounce, parallax, motion on idle.
- **Reduced motion.** The `AnimationsEnabled` policy field (0x0115) already
  exists and `animation::enabled` already lands every animation at its end
  when it's off. Settings gets the row, named `Reduce motion`.

## Window chrome

**Proposed, owner to confirm** (controls on the right). The red, yellow and green circles on the left are macOS's
identity, not ours.
- **Title bar.** 36 high.
  - The title is `type.caption` Geist 500, left-aligned after a 16 px
    inset, in `text` when focused and `text.2` when not. (`text.3` was
    asked first; the frame proof found it under 4.5:1 in Paper.)
  - The bar is the window's own ground, with the rule under it. A bar a
    shade darker than the ground failed the same proof in Paper.
  - The app may put its own accessory (tabs, search) after the title.
- **Controls on the right.** Minimize, maximize and close.
  - Each is a 1.5-stroke line glyph, 16 px, in a 36 × 28 hit area.
  - Hover gives a `raised` wash.
  - Close hover is a `bad` fill with `on.bad` glyph.
  - No colour at rest, so meaning never rests on colour alone.
- **Focus.**
  - The focused window carries a 1 px cyan rule along the top of its frame.
    It is the site's thin rule, and the one thing that tells you where your
    keys go.
  - Unfocused windows lose the rule and dim their title.
  - Today focused and unfocused windows look identical.
- **Frame.**
  - A 1 px `rule` edge and 10 px corners. Corners are smaller than the
    tile's: windows are tools, not tiles.
  - One shadow, for windows and menus only. Nothing inside a window casts a
    shadow.
- **Resize.** Every edge and corner resizes, not only right and bottom.
  The cursor changes shape over each.
- **Keys.**
  - Super+Up maximizes.
  - Super+M minimizes.
  - Ctrl+W closes, where the app has no tab to close first.
  - Alt+Tab and Alt+Shift+Tab switch windows, with a strip of titles.

This moves desktop_proofs' frame tests (button width 12, the titlebar
height, HAIRLINE) and wm_proofs' menu-bar test. Each moved proof gets its
mutant again in the chrome unit.

**Built in ui/chrome (host-tested):**
- The bar, the title, the controls, the hover and close states, the frame
  colours from the live roles in every scheme and in high contrast, the
  corners, and the focus rule and dimmed title as frame states.
- The title draws at 17 while the toolkit's 17 px floor stands. It moves to
  `type.caption` with the floor, in the type unit.
- The focus state needs a per-window focus event from the WM. Resize from
  every edge, cursor shapes and the keys above need the WM, the input
  router and the compositor. Those owners are being settled with the Lead.

## Voice

The site sets it: say only what is true and can be checked, a number before
an adjective, state what is unfinished. Inside the OS that becomes the
rules below. The copy check, the third unit, enforces the first three.

1. **No hype words.** None of seamless, unlock, empower, elevate, dive in,
   supercharge, effortless, magic, revolutionary or next-level. No emoji
   and no `✨`. No exclamation marks in UI strings.
2. **Sentence case.** Buttons are verbs: `Install`, `Remove`, `End process`.
   Not `OK` where a verb fits.
3. **Errors say what happened and what to do.**
   - `Couldn't reach the settings service. Try again, or restart Settings.`
   - Not `short reply`.
   - Not `reading the registry needs AttestRead`. That becomes:
     `About can't read the capsule registry: it wasn't given the AttestRead
     capability.`
4. **Name the mechanism, not the feeling.** `Signed with Ed25519, key
   4f1a…` and not `Secure`.
5. **Say what can't be undone, once.** On the button and on the line above
   it, not in a paragraph.
6. **Numbers come with units.** `412 MiB`, `1.25x`, `3 of 17`.
7. **The name** is NØNOS in UI text, NONOS only where Ø can't be typed.

## The no-AI-traces check

The Lead's list, checked on every screendump in every unit's report:

- **Visual:**
  - no purple or blue gradient washes;
  - no glass or neon glow as decoration (the installer's mark is the ruled
    exception);
  - no sparkle icons, no emoji;
  - no one radius and one shadow for everything;
  - no centred hero inside apps;
  - no stock mesh backgrounds;
  - no filler illustration;
  - no lorem or placeholder.
- **Copy:** the banned words, `✨`, exclamation marks and title case, by
  static check.
- **Consistency:**
  - the same spacing, type sizes, icon weight and states in every app;
  - a side-by-side of each app's screendump against this page in its
    report.

## References

These are the qualities each is held to, not things to copy.

| Reference | The quality held to | Where NØNOS differs |
|---|---|---|
| The site, nonos.software | exact copy, thin rules, numbered mono labels, black and cyan | the OS is denser and keyboard-first |
| Teenage Engineering product UI (OP-1 and field) | a few strong marks, mono labels, nothing decorative | no skeuomorphic texture |
| Linear | speed, keyboard everything, quiet chrome, one accent | no purple gradients, no marketing tone in product |
| Swiss/International typographic posters | grid, flush-left, type as structure | on screen, with interaction states |
| Plan 9 / acme | text is the interface, nothing hidden | modern type, pointer and keyboard both |
| Ableton Live | dense information that stays legible, flat colour as state | dark Ink ground, cyan instead of orange |
| Bloomberg Terminal | data first, mono numbers, tabular alignment | humane spacing outside data views |
