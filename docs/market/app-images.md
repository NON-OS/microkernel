# App images: building and re-pinning one

An app image is an app and its Alpine closure, cut into members and listed
by one measurement, the BLAKE3 root of its chunk list. The entry under
`nonos-data/market/images/` is unsigned: it names the measurement, the
mirror URLs and what went in. The release seal signs it and lists it in the
market index, so `index.json` and `index.bin` are never edited by hand.

## When the pin catches a change

The measurement moves when any input moves:

- Alpine serves other bytes for a package in the closure: a new revision,
  or a package rebuilt under the same version, whose signed index then
  names a new checksum;
- the pack tool cuts a package at other places, because the size a pack
  takes changed (`tools/nonos_market_catalogue/pack_size.py`, which follows
  the device's pack format);
- the files the image ships, or GTK's caches, change;
- a file the NONOS image itself carries in its Linux tree (the "linux"
  group of `tools/nix/store.json`) is in the closure, or stops being one:
  the device refuses any package that would replace one of those files
  (`install/place/check.rs`) and fails the whole install, so the image is
  built without that entry, said by name as `<package>: <path> left out:
  the NONOS image carries it` (`tools/nonos_linux_pack/image_carried.py`).

The build is still reproducible: the same signed indexes, the same tools,
the same command and the same shipped files give the same bytes on any
host. Re-pinning is building it again and committing the new entry.

## The build

Run it in the dev shell, so `b3sum` is on the PATH; without it the hash
falls back to `tools/nonos_linux_pack/blake3.py`, which is correct and takes
hours on an image this size.

```
cd tools
python3 -m nonos_linux_pack image --package libreoffice \
  --with font-noto --with font-liberation \
  --mask 'WAYLAND FILES_HOME FILES_USER EXEC' \
  --program /usr/lib/libreoffice/program/soffice.bin \
  --files ../userland/capsule_market/image-files/libreoffice \
  --out OUT --cache CACHE
```

Nothing in LibreOffice's closure depends on a font, so the two `--with`
lines are part of the recipe: without them the image has no fonts and
draws no text. The shipped files are LibreOffice's Work path,
`$(home)/Documents`, in a registry file under its share/registry
(`nonos-files-20be4d6acbd6e3c2`).

OUT gets `<measurement>.image`, `<measurement>.chunks` and
`<measurement>.json`. Copy the JSON over
`nonos-data/market/images/linux.nonos-image-libreoffice.json` and check the
diff: the measurement, the URLs, `image_size`, `list_size` and the members
of the package that changed, and nothing else. The `box`, and its
`program_blake3` above all, must not move unless the program did. Then run
`python3 -m unittest nonos_market_catalogue.test_image_index
nonos_market_catalogue.test_image_listing` in `tools`.

## The size

The device takes an image up to 1 GiB (`nonos_app_image` MAX_IMAGE) and
fetches it in 4 MiB chunks. An image is kept no larger than the one the
mirror already serves for it, so a re-pin never asks more of the content
host.

## Publishing a new image

Place `<measurement>.image` and `<measurement>.chunks` on the package mirror
under `linux/`, then seal: the seal signs the new entry and lists it.

## LibreOffice 7.6.7.2-r0, re-pinned 2026-11-05

`libreoffice-common-7.6.7.2-r0` is the same package it was (signed checksum
`Q13CgsPJoBlpw31hVP96ZL64r/7u8=`, 158896374 bytes, 342937600 installed). What
changed is the pack format: since the per-page digest change a NONOSPK2 pack carries a 16-byte
digest for each 4 KiB page and starts each file of a page or more on a page,
and `pack_size.py` counts that. libreoffice-common is cut into six members,
each the most its 64 MiB pack holds, so the larger count moves the cuts
after its second member. Cutting today's package with `pack_size.py` as it
was before that change gives exactly the six cuts of the old entry; with it
as it is now, the old third and fifth members no longer fit. The package
list is the same 245, and every other member, the shipped files among
them, has the size it had. Two independent builds from the same inputs
gave byte-identical images, 0030b1a5..., matching the committed entry.

| | before | after |
|---|---|---|
| measurement | `7d7092f2…b92049` | `0030b1a5…1ed258` |
| image | 225333756 bytes | 225047900 bytes |
| members | 252 | 252 |
| chunk list | 1760 bytes | 1760 bytes |

## LibreOffice 7.6.7.2-r0, re-pinned again 2026-11-05

0030b1a5 never installed. On a development guest with the image served
by `tools/nonos-mirror-relay --content`, every chunk came and verified,
then the install failed at python3, a package of the closure:
`[LINUX] python3: refused member usr/bin/python3: the image carries it,
or a file under it; nothing of it is written`. The NONOS image carries
its own `/usr/bin/python3`. The image is now built without that one
symlink; Alpine's `python3.12` and `libpython3.12.so.1.0`, which
LibreOffice loads, stay. Every other member is the one it was, and the
Telegram image built the same way comes out as its pin, 720f7756..., as
its closure holds none of the image's files.

| | before | after |
|---|---|---|
| measurement | `0030b1a5…1ed258` | `21c53553…b68538` |
| image | 225047900 bytes | 225048892 bytes |
| members | 252 | 252 |
| chunk list | 1760 bytes | 1760 bytes |
