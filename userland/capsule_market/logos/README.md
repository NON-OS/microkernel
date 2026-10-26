# Catalogue app logos, as upstream ships them

Each file here is an upstream project's logo, vendored byte for byte from
the URL HANDOFF/catalogue/logos-licences.md gives for it. The recipe in
../linux-catalogue.json names it as `logo`, the lock pins its BLAKE3 as G,
and the raster made from it once, committed at
nonos-data/market/logos/linux.<package>.rgba, is pinned as R and signed by
the operator index as the listing's logo_hash. The market capsule serves a
raster only when it hashes to that logo_hash.

Which logo may be shown is the record's to say, not this directory's: git
is CLEAR; the others are UNCLEAR rows the owner ruled on 2026-10-25 ("Use
them anyway"). age is not here: its terms allow only unaltered use, and no
tile shows its logo at its own pixels, so it keeps a NONOS icon.

Credits and terms that travel with the files:

- git.svg: Git Logo by Jason Long, CC BY 3.0
  (https://creativecommons.org/licenses/by/3.0/). Git™ is a trademark of
  Software Freedom Conservancy; NONOS is not affiliated with the Git
  project.
- vim.png: runtime/vim48x48.png from Vim 9.1.0707, shown unmodified at its
  own 48x48 pixels under licence I of Vim's licence, whose text is
  vim-licence.txt (runtime/doc/uganda.txt at the same tag).
- helix.svg (helix 24.03), micro.svg (micro v2.0.13), kakoune.svg (Kakoune
  v2024.05.18), bat.svg (bat v0.24.0), nnn.svg (nnn v4.9), fzf.png,
  flac.gif, lame.gif, profanity.png (Profanity 0.14.0): each project's own
  logo, shown under the owner's ruling. If a project objects or states
  terms, its row moves to FORBIDDEN, its file and raster are removed, and it
  falls back to its NONOS icon at the next seal.

To remake the rasters (a maintainer, never the build), from the tree's root:

    PYTHONPATH=tools python3 -m nonos_market_catalogue.logos --rasterise

It needs Pillow and rsvg-convert. Then review, run the catalogue with
--update-lock, and commit the rasters and the lock together.
