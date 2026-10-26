# Linux app logos in the App Store (proposed, catalogue session)

Version 0.1, 2026-10-25. For the Lead, the owner, UI/UX and track B. Owner
decision (verbatim): "Real logo where licence allows". The record is
logos-licences.md: 1 CLEAR (git), 11 UNCLEAR, 6 with no logo. Claims are
marked (design) or (host-read: file:line).

## 1. Data: recipe, record, lock

- A recipe today has only these keys: package, licence, mask, dests, calls,
  name, text. Any other key is refused (host-read:
  tools/nonos_market_catalogue/recipes.py:35, :50-52). Add an optional `logo`:
  a tree path to the upstream file, vendored byte for byte, e.g.
  `userland/capsule_market/logos/git.svg`. It is resolved inside the tree as
  `dests` is (host-read: recipes.py:59-62) (design).
- The record is logos-licences.md. A recipe may name a logo only if its row
  starts with CLEAR (design).
- The lock pins V, C and S per package, plus B, the BLAKE3 of the .apk
  (host-read: lock_file.py:86-88). A pin with any other key set is refused
  (host-read: lock_file.py:109). lock.write writes exactly these four
  (host-read: lock.py:56-60). The shape is in
  userland/capsule_market/linux-catalogue.lock.json:1-20.
- Proposal: lock v3 adds `G` (the BLAKE3 of the vendored source) and `R` (the
  BLAKE3 of the raster the device gets), "" for no logo. Only
  `--update-lock` moves them, as for B (host-read: lock.py:16-22) (design).
- Rasterising: the house tool, userland/assets/icons/build.sh, makes alpha
  masks at 192px (host-read: build.sh:7-12) and RGBA for the one colour
  brand mark (host-read: build.sh:14-17). Real logos keep their colours, so
  they become RGBA at the size or sizes UI/UX names. ImageMagick output can
  differ between versions, so a maintainer step makes the raster and it is
  committed; the build checks it against R and does not remake it. Nothing
  is fetched at build time or at run time (design).

## 2. Wire: signed index, then bytes to the image

- A Linux entry is built in catalogue.one, with the operator key as
  publisher (host-read: catalogue.py:41-56), from records.entry (host-read:
  records.py:57-70). The encoder writes entry fields in a fixed order, then
  the releases (host-read: marketplace_abi/src/codec/encode_entry.rs:22-42).
  ReleaseExt holds commerce, guest_policy, chunk_root, content_len, anchors
  and attest (host-read: types/release_ext.rs:25-38). The encoder writes
  schema 3 only; the reader accepts 2 or 3 (host-read: codec/schema.rs:24-31).
- Proposal: an entry field `logo_hash: [u8; 32]`, zero for none, after
  `description` (encode_entry.rs:29), in schema 4. The JSON side gains it
  beside `description` (host-read: nonos-mk/src/json/schema/entry.rs:30).
  It belongs on the entry, not in ReleaseExt, because the logo is the
  listing's, not a release's. The operator's index signature covers it
  (design).
- Bytes today: the seal signs nonos-data/market/index.bin (host-read:
  tools/nonos_seal/market.py:30, :109-121). The build copies it to target/market
  (host-read: tools/nix/capsules.nix:94-97), and capsule_market embeds it
  (host-read: capsule_market/src/baseline.rs:22). Only the crates that name it
  get it (host-read: tools/nix/inputs.py:79-83).
- Proposal: commit the rasters at nonos-data/market/logos/<listing_id>.rgba,
  copy them in the same signedInputs step, and add a MARKERS entry. Embed
  them in capsule_market, which checks each BLAKE3 against the signed
  logo_hash before serving it. The seal adds the directory to SOURCES so the
  serial moves (host-read: market.py:65-74) and to `committed()` so a keyless
  seal keeps the logos with the index (host-read: market.py:35-43, :97-100).
  It stops when the committed index and the logos disagree (design).
- Store to market: OP_GET_LOGO = 9, after OP_GET_LINUX_POLICY = 8 (host-read:
  market_proto/src/ops.rs:19-27), handled beside get_app (host-read:
  capsule_market/src/server/runner.rs:61-71, handlers/get_app/handle.rs:29).
  The reply is the size and RGBA, or "none". The store asks once per listing
  and keeps the answer, as it does for Known (host-read:
  capsule_app_store/src/store/listing.rs:44-64). Putting the logo in the App
  reply instead (host-read: market_proto/src/app.rs:25-30) costs more: that
  reply is asked for every listing (listing.rs:49-51) (design).

## 3. Store side (the look is UI/UX's)

A card draws a 36px tile tinted by source, holding the name's initial
(host-read: store/ui/card_tile.rs:27-33, ui/metrics.rs:42). The detail pane
opens with the name and has no image (host-read: ui/detail.rs:36-38). There is
one dark palette (host-read: store/theme.rs:16-45). The likely places are the
card tile and a larger mark in the detail pane. UI/UX decides size,
placement, the tile backing for colour logos on the dark palette (helix ships
logo_dark and logo_light), future themes, and where git's credit ("Git Logo by
Jason Long, CC BY 3.0") and ™ go (design).

## 4. Fallback: NONOS-drawn icons

Every UNCLEAR, FORBIDDEN or no-logo app gets an icon in the desktop's own
language: an SVG in userland/assets/icons made into an alpha mask and tinted
cyan on a rounded tile (host-read: capsule_desktop_shell/src/render/icons.rs:26-28,
icons/badge.rs:21-35, assets/icons/build.sh:8-12). Installed tools already
work this way, with a generated tile when none exists (host-read:
render/launchpad/tool_icons.rs:14-40). Proposal: one glyph per kind (editor:
helix, micro, kakoune, vim; files: eza, nnn, ncdu; search: fzf; version
control: git, tig, lazygit; keys: age, minisign; audio: flac, lame; chain:
geth; chat: profanity), in the toolkit mask table (host-read:
toolkit/src/icons/table.rs, icons/mask.rs:20-22), which the store links
(host-read: capsule_app_store/Cargo.toml:31). The store draws it without
asking the market, so no logo ever blocks a row; the initial on its tint
stays the last resort. The launcher may reuse it (desktop session) (design).

## 5. Checks

- Static (design), a test beside test_documents.py (host-read:
  test_documents.py:32-45). For each recipe with `logo`: its row is CLEAR,
  the lock has G and R, the source hashes to G, the committed .rgba exists
  and hashes to R, and the committed document's logo_hash is R. For each
  recipe without one: G and R are "", logo_hash is zero, and no file exists.
- Mutants (design): one per rule in mutants.json's name/file/old/new form
  (host-read: mutants.json:1-30). They drop the CLEAR check, accept
  UNCLEAR, skip G, skip R, and let a logo-less recipe carry bytes. The
  logos directory and logos-licences.md join COPY (host-read:
  test_mutants.py:34-37).
- Rust (design): schema 4 round trips; a market serving a logo whose
  BLAKE3 is not the signed one fails a test.
- Booted (design), as the profanity run did (host-read:
  docs/market/evidence/catalogue/profanity-35a5a4660/drive.steps): screenshot
  the App Store's Linux tab. git's tile shows the logo, every other tile its
  NONOS icon, and serial has no `[MARKET]` logo refusal. A pixel probe of the
  tile is checked against the committed raster.

## 6. Open questions

- Owner, a ruling per UNCLEAR row: helix, micro, kakoune, vim, bat, nnn,
  fzf, age, flac, lame, profanity. Each needs a written grant covering
  listing use and resizing. age's grant would come from age-logo@filippo.io;
  its current terms forbid any altered form. vim could use
  runtime/vim48x48.png unaltered under its licence I, if the owner accepts
  that reading.
- Owner: git is CLEAR with conditions (™, credit, no implied association).
  Accept them, or draw git in NONOS style for uniformity?
- Lead: schema 4 for one field now, or with the next schema change?
  Should logos-licences.md move under plan/?
- UI/UX: sizes, placement, colour-logo backing, credit, fallback drawing.
- Track B: OP_GET_LOGO or an App-reply field; embedding or a boot-read file.
  May a newer operator index (host-read:
  capsule_market/src/boot_index.rs:28, :40-45) name logos the image lacks?
  Proposed: yes, and the store shows the fallback.

## 7. As built (catalogue/logos, 2026-10-25)

Rulings: Lead, schema 4 with `logo_hash`, OP_GET_LOGO = 9, the static
check, mutants, a booted-screenshot plan. Owner, real logo where the
licence allows; UNCLEAR rows "use them anyway" (written into each row of
logos-licences.md); a NONOS-drawn icon otherwise. What landed, and where it
differs from sections 1 to 5 (host-tested unless marked design):

- Recipe `logo`, lock v3 (G, R), the record check, the maintainer step
  (`python3 -m nonos_market_catalogue.logos --rasterise`, Pillow and
  rsvg-convert) as section 1. A raster is square RGBA with no header, its
  side recovered from its length: a square PNG or GIF no larger than 48 px
  keeps its own pixels (vim 48, profanity 32); anything else is fitted into
  36 px, the card tile, and centred on a clear square.
- Schema 4 as section 2: marketplace_abi writes 4 only and reads 2 to 4;
  an entry document is one entry of the current schema. The committed
  index.bin is still the operator's schema 2 one (the Qwen tiers); the
  catalogue's documents under nonos-data/market/catalogue/ carry
  `logo_hash` now, and the index signs it at the next keyed seal (design).
- Differs: the rasters are embedded with include_bytes straight from
  nonos-data/market/logos/ (userland/capsule_market/src/logos/table.rs),
  not copied in signedInputs under a MARKERS entry. The seal never writes
  them, so they are ordinary tracked inputs and tools/nix/inputs.py names
  them for capsule-market as it does any file a crate reads.
- The seal (tools/nonos_seal/market.py) adds the logos and the record to
  SOURCES, keeps a keyless seal's index only when the logos are committed
  too, and stops when the index's JSON names a logo_hash that is not the
  committed raster's BLAKE3. A raster the index does not name yet is
  served as none. CAPSULE_SOURCES in tools/nonos_seal/__main__.py still
  excludes nonos-data/market whole, so a logo change is caught by the
  enrollment check, not by that diff (design: the Lead may want
  nonos-data/market/logos taken out of that exclusion).
- Wire: OP_GET_LOGO takes the listing id; the reply is one byte, the side,
  then side * side * 4 bytes, or a single 0 for none
  (userland/market_proto/src/logo.rs). The market answers E_KEYREJECTED and
  logs `[MARKET] logo refused for <id>: not the signed BLAKE3` when the
  bytes it carries are not the signed ones.
- Store: asked once per Linux listing, beside the description; the tile
  draws the logo at its own pixels centred on the 36 px tile (a 48 px logo
  overhangs it by 6 px each side, inside the card), else a NONOS icon by
  kind, else the initial. The NONOS icons reuse the toolkit's existing
  glyphs rather than new drawings (userland/capsule_app_store/src/store/ui/
  card_icon.rs): editor, files, storage, search, terminal (version
  control), lock (keys), audio, wallet (geth), network (chat). New glyphs,
  the detail-pane mark, the credit line and the ™ are UI/UX's (design).
- age keeps its NONOS icon: its terms allow only unaltered use and its
  logo.png is 1246x592.

### Booted-screenshot plan (design, not run)

1. Build a dev image of the branch head: `make dev-image` (qemu-dev). It
   seals with tools/nonos-dev-image's throwaway keys in its own copy of the
   tree, so the dev index it signs is schema 4 and names each logo_hash.
   Never with the checkout's .keys/.
2. Boot it with `tools/nonos-drive --qemu-arg=--fresh --qemu-arg=--lane
   --qemu-arg=iommu=intel` and the setup walk of
   evidence/profanity-35a5a4660/drive.steps up to `shot:d2`.
3. Open the App Store (its dock icon: take a `shot`, then `move` to it and
   `click`), wait 8 s, press `key:right` twice to reach the Linux tab
   (All, NONOS, Linux), wait 10 s for the per-listing asks, `shot:linux-tab`,
   then `key:down` through the list and `shot:linux-tab-2` for the rest.
4. Pass when: git, helix, micro, kakoune, vim, bat, nnn, fzf, flac, lame
   and profanity show their logos; age, eza, ncdu, tig, lazygit, minisign
   and geth their NONOS icons; `grep -c 'logo refused' serial.log` is 0.
5. Pixel probe: crop git's tile from linux-tab.png (36x36 at the tile's
   origin) and compare it with nonos-data/market/logos/linux.git.rgba
   composited over the theme's TILE_LINUX colour; every channel within 2.
