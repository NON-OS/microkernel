# Linux app logos: licence and trademark record

Date checked: 2026-10-25. The record that logos-design.md's checks read. One
row per app in userland/capsule_market/linux-catalogue.json (18 recipes:
the 17 in the 0.9.3 set plus profanity).

The rule (owner, verbatim): "Real logo where licence allows". A row is
CLEAR only when the logo's own licence AND the project's trademark terms
explicitly permit showing it in a third-party store listing, including
the change we make (rasterise and resize). Silence counts as UNCLEAR.
UNCLEAR and FORBIDDEN rows are the owner's to rule on; until then the
app gets a NONOS-drawn icon. "No logo" rows get a NONOS-drawn icon and
need no ruling.

Sources were read with WebFetch/WebSearch only (GitHub pages, project
sites, two raw text files). Nothing was downloaded to the tree and no one
was contacted. Quotes are as the pages gave them on that date.

## Table

| App | Logo exists? (official source) | Licence the logo itself is under | Trademark / brand policy | Verdict |
|---|---|---|---|---|
| helix | Yes: logo.svg, logo_dark.svg, logo_light.svg at the root of github.com/helix-editor/helix. README: "Thanks to @jakenvac for designing the logo!" | No notice on the file. A collaborator (kirawi, 2024-06-14, accepted answer in discussion #10958): "It is licensed under the MPL 2.0 license". Repo licence MPL-2.0. | None found. MPL-2.0 s2.3 withholds logo rights (quoted below). | UNCLEAR; owner 2026-10-25: "Use them anyway" |
| micro | Yes: assets/micro-logo.svg, micro-logo-mark.svg, micro-logo-drop.svg in github.com/micro-editor/micro (README shows micro-logo-drop.svg) | No notice on the files. Repo LICENSE is MIT for "this software and associated documentation files"; it does not say it covers artwork. | None found. | UNCLEAR; owner 2026-10-25: "Use them anyway" |
| kakoune | Yes: doc/kakoune_logo.svg in github.com/mawww/kakoune; README: "Thanks to p0nce for designing the Kakoune logo." Also kakoune.org/img/kakoune_logo_32.png | No notice on the file. Repo is under UNLICENSE (public-domain dedication for "this software"). It does not say whether the logo, made by a third party, is dedicated too. | None found. | UNCLEAR; owner 2026-10-25: "Use them anyway" |
| vim | Yes: runtime/vimlogo.svg (and .eps .pdf .gif .xpm .cdr) and runtime/vim16x16, vim32x32, vim48x48 (.png .gif .xpm) in github.com/vim/vim. vim.org/logos.php: "The official logo is the word "Vim" over a green diamond." | No notice on the files. They ship inside Vim, so the Vim licence applies (runtime/doc/uganda.txt). Copying unmodified parts is allowed with the licence text. Resizing makes a "modified" part, which triggers conditions II 1-2. | None found. vim.org/logos.php gives no terms. | UNCLEAR; owner 2026-10-25: "Use them anyway" |
| bat | Yes: doc/logo-header.svg in github.com/sharkdp/bat | No notice on the file. Repo is MIT OR Apache-2.0 at the user's option. Neither says it covers artwork. Apache-2.0 s6 withholds trademark rights. | None found. | UNCLEAR; owner 2026-10-25: "Use them anyway" |
| eza | No logo found. The README shows a screenshot (docs/images/screenshots.png) and the org has an avatar. REUSE.toml names no image file. | n/a | None found. | NO LOGO (NONOS-drawn) |
| nnn | Yes: misc/logo/logo.svg, logo-64x64.png, logo-128x128.png in github.com/jarun/nnn | No notice in misc/logo/. Repo licence BSD-2-Clause; it does not say it covers artwork. | None found. | UNCLEAR; owner 2026-10-25: "Use them anyway" |
| ncdu | No logo found. dev.yorhel.nl/ncdu shows only a decorative /img/scissors.png, which it does not call a logo. | n/a (the software is MIT) | None found. | NO LOGO (NONOS-drawn) |
| fzf | Yes: junegunn.github.io/fzf shows /android-chrome-192x192.png with alt "Logo". The README header is raw.githubusercontent.com/junegunn/i/master/fzf-color.png, which is not labelled a logo. | No notice. The site footer says "Copyright © 2025 Junegunn Choi". Repo licence MIT; it does not say it covers artwork. | None found. | UNCLEAR; owner 2026-10-25: "Use them anyway" |
| git | Yes: git-scm.com/community/logos (git-scm.com/downloads/logos redirects there) | Stated on that page: "Git Logo by Jason Long is licensed under the Creative Commons Attribution 3.0 Unported License." | git-scm.com/about/trademark (Software Freedom Conservancy). The logos are "Marks" under s1. s2.2 allows use without written permission "To refer to the Git software in substantially unmodified form" and "To identify the Git software as a distinct component of a software offering". | CLEAR, with conditions (below) |
| tig | No logo found in github.com/jonas/tig (README has badges only; the visible listing has no image files) | n/a (GPL-2.0) | None found. | NO LOGO (NONOS-drawn) |
| lazygit | No logo identified. The README's header image (user-images.githubusercontent.com/8456633/174470852-....png) has no alt text and is not called a logo. Other images are sponsor and Discord marks. | n/a. If the header image is a logo, nothing states its terms. Repo licence MIT. | None found. | NO LOGO (NONOS-drawn) |
| age | Yes: logo/logo.svg, logo.png, logo_white.svg, logo_white.png in github.com/FiloSottile/age | logo/README.md: "Copyright 2021 Filippo Valsorda". Permission only for unaltered use, revocable (quoted below). Designed by Studiovagante. | The logo README is the brand terms. Repo licence BSD-3-Clause has a no-endorsement clause. | UNCLEAR (FORBIDDEN for any altered form); owner 2026-10-25: "Use them anyway", but only unaltered: no tile can show logo.png at its 1246x592 pixels, so age keeps its NONOS-drawn icon |
| minisign | No logo found (README has two badges only; jedisct1.github.io/minisign has no image) | n/a (ISC) | None found. | NO LOGO (NONOS-drawn) |
| flac | Yes: xiph.org/flac shows /flac/images/logo130.gif (alt "FLAC Logo"). Older copies are under doc/html/images/ in the FLAC source (gitlab.xiph.org). wiki.xiph.org/Logos lists FLAC logos (503 when checked, so its terms were not read). | No notice on the image. The FLAC README splits the licence: libraries Xiph BSD-like, programs GPL, "documentation is licensed under the GNU Free Documentation License". It does not say whether images under doc/ count as documentation. | No formal policy found. xiph.org/about: "The Xiph Fish Logo and the Vorbis.com many-fish logos are trademarks (™) of Xiph.Org." It does not name the FLAC logo. Informal 2003 list posts (not official) say logo use "currently needs permission". | UNCLEAR; owner 2026-10-25: "Use them anyway" |
| lame | Probably: the LAME docs and site use images/logo.gif (alt "LAME Official Logo", 358x231, per mirrored doc/html). lame.sourceforge.io returned 403, so this was not confirmed at the source. | Unknown: no notice found. The software is LGPL. | None found. | UNCLEAR; owner 2026-10-25: "Use them anyway" |
| geth | No Geth logo found. The github.com/ethereum/go-ethereum README has none, and geth.ethereum.org shows "go-ethereum" as plain text. An old gh-pages site had a mascot.png, which is not a current mark. The Ethereum diamond belongs to the Ethereum brand, not Geth. | n/a | ethereum.org/en/terms-of-use: Foundation marks "You must not use such marks without the prior written permission of the Foundation"; site content is CC BY 4.0 "Unless otherwise marked". Not applicable without a Geth logo; we should not stand in the diamond for it. | NO LOGO (NONOS-drawn) |
| profanity | Yes, according to the other session: icons/proIcon.png in its 0.14.0 source tree. Today's master icons/ lists profanity.svg, profanity-msg-bubble.png and profanity-new-message.png (seen 2026-10-25, no notice there). | GPL-3.0-or-later covers the tree (the other session's finding, not re-checked here). It does not say it covers artwork. | None found (the other session's finding). | UNCLEAR; owner 2026-10-25: "Use them anyway" |

Count: CLEAR 1 (git). UNCLEAR 11 (helix, micro, kakoune, vim, bat, nnn,
fzf, age, flac, lame, profanity). No logo 6 (eza, ncdu, tig, lazygit,
minisign, geth). FORBIDDEN: none as a whole, but age forbids every altered
form.

## Clauses, per row

**git (CLEAR, with conditions).** Copyright: "Git Logo by Jason Long is
licensed under the Creative Commons Attribution 3.0 Unported License"
(git-scm.com/community/logos). CC BY 3.0 permits adaptation, which covers
rasterising and resizing, with credit. Trademark (git-scm.com/about/trademark
s2.2): "You may use the Marks without prior written permission (subject to
the other sections): 1. To refer to the Git software in substantially
unmodified form. "Substantially unmodified" means built from the source
code provided by the Git Project, possibly with minor modifications
including but not limited to: ... changes required for compatibility with
a particular operating system distribution, or the inclusion of bug-fix
patches. 2. To identify the Git software as a distinct component of a
software offering." Conditions that bind us:
- Credit "Git Logo by Jason Long, CC BY 3.0" somewhere the listing reaches,
  such as the detail pane or an About/credits page.
- s2.1: "The first prominent mention of a Mark should be immediately
  followed by a symbol for ... an unregistered trademark (™)". This is
  waived only "where such marks are not required to protect the
  intellectual property rights", and a store listing is not clearly one of
  those contexts.
- s2.3: not "In any way that indicates a greater degree of association
  between you and the Git Project than actually exists" and not "In any
  way that indicates that Git favors one distribution, platform, product,
  etc. over another".
- s2.5: no merchandise.
The policy has no clause on app stores or on altering the logo files, but
s2.2 permits the use and CC BY permits the change. Our git is Alpine's
build in a box that refuses its remote helpers, unmodified as software, so
it is "substantially unmodified".

**helix (UNCLEAR).** MPL-2.0 s2.3 (the licence text): "This License does
not grant any rights in the trademarks, service marks, or logos of any
Contributor (except as may be necessary to comply with the notice
requirements in Section 3.4)." The licence the collaborator names for the
logo itself withholds the right to use it as a logo. There is no
trademark policy, and the only licence statement comes from a
collaborator, not a maintainer.

**bat (UNCLEAR).** Apache-2.0 s6 (the licence text): "This License does
not grant permission to use the trade names, trademarks, service marks, or
product names of the Licensor, except as required for reasonable and
customary use in describing the origin of the Work and reproducing the
content of the NOTICE file." MIT is silent on marks. No notice covers
doc/logo-header.svg.

**micro, nnn, fzf, kakoune, lame, profanity (UNCLEAR).** A logo exists,
but nothing states the terms for the image or for the project's name and
mark. The repo licences (MIT, BSD-2-Clause, UNLICENSE, LGPL, GPL) license
"the software". None says it covers the artwork or grants mark rights.

**vim (UNCLEAR).** Vim licence I (runtime/doc/uganda.txt): "You can also
distribute unmodified parts of Vim, likewise unrestricted except that they
must include this license text." Using runtime/vim48x48.png pixel for pixel
would be an unmodified part. Any resize is a modified part, under II:
"1) This license text must be included unmodified. 2) The modified Vim
must be distributed in one of the following five ways" (a: say how to
contact us, and give the maintainer the changes on request; c: ship the
changes). There is no trademark or brand statement anywhere. Whether a
store tile is "distributing a part of Vim" at all is the owner's call.

**age (UNCLEAR; FORBIDDEN for any altered form).** logo/README.md:
"Permission is granted to use the logos as long as they are unaltered, are
not combined with other text or graphic, and are not used to imply your
project is endorsed by or affiliated with the age project. This permission
can be revoked or rescinded for any reason and at any time, selectively or
otherwise. If you require different terms, please email
age-logo@filippo.io." Our rasterise and resize alters the logo, and a
tinted tile beside the name may count as "combined with other text or
graphic". Only logo.png at its own pixels, alone, might pass. The
permission is also revocable.

**flac (UNCLEAR).** xiph.org/about marks only the Xiph fish and the
Vorbis.com fish as Xiph.Org trademarks. No document grants use of the FLAC
logo. The 2003 posts on lists.xiph.org (vorbis-dev 2003-June/016305) say:
"a piece of software wishing to display one of Xiph's logos currently
needs permission". The writer also says he is "neither a lawyer or a
Xiph.org representative", so this is not policy.

**geth (no logo).** For the record, ethereum.org terms of use: "You must
not use such marks without the prior written permission of the
Foundation". The Ethereum diamond is not Geth's logo and is not used.

## Side findings, not logo questions

- geth: the go-ethereum README says the binaries ("all code inside of the
  `cmd` directory") are GPL-3.0, and only the library is LGPL-3.0. The
  recipe says LGPL-3.0-or-later (host-read:
  userland/capsule_market/linux-catalogue.json:34), which is Alpine's
  field, and catalogue.py:43 checks the recipe against that field. The
  Lead may want the listing's licence to name the binary's licence.
- eza: upstream main is now EUPL-1.2 (README, REUSE.toml). The pinned
  0.18.16 is MIT, as the recipe says. A version bump would need the
  allowlist (tools/nonos_market_catalogue/licences.py) to accept EUPL-1.2.

## Owner rulings, 2026-10-25

Asked in the catalogue session with the question tool; the answers verbatim:

- The 11 UNCLEAR rows (helix, micro, kakoune, vim, bat, nnn, fzf, age, flac,
  lame, profanity): "Use them anyway". Each shows its project's real logo.
- git: "Yes, with the conditions (Recommended)". The real logo, with the
  credit "Git Logo by Jason Long, CC BY 3.0", a ™ on first prominent
  mention, and no wording that implies association with the Git project.

How the catalogue carries out the first ruling:

- age's own terms are the only explicit restriction found. They allow use
  "as long as they are unaltered, are not combined with other text or
  graphic", and the permission is revocable. So age shows logo/logo.png at
  its own pixels, alone. Where the store can only show a resized or tinted
  tile, age shows its NONOS-drawn icon instead.
- vim shows runtime/vim48x48.png unmodified, under its licence I, with the
  licence text reachable from the listing.
- Each of the 11 keeps its row's source and terms here. If a project later
  objects or states terms, the row moves to FORBIDDEN and the app falls back
  to its NONOS-drawn icon at the next seal.
- lame's source could not be reached (403). Its row stays unconfirmed until
  the logo file is read from the LAME source tree.

## What the catalogue ships, 2026-10-25 (catalogue/logos)

The Verdict cell is what tools/nonos_market_catalogue/logos.py reads: a
recipe may name a logo only when its row's verdict starts with CLEAR, or
starts with UNCLEAR and carries the owner's ruling as written above
(`owner 2026-10-25: "Use them anyway"`). Each vendored file is the upstream
file byte for byte, under userland/capsule_market/logos/, fetched from:

| App | File | Fetched from | Shown as |
|---|---|---|---|
| git | git.svg | https://git-scm.com/images/logos/downloads/Git-Icon-1788C.svg | resized to 36 px |
| helix | helix.svg | https://raw.githubusercontent.com/helix-editor/helix/24.03/logo.svg | resized to 36 px |
| micro | micro.svg | https://raw.githubusercontent.com/micro-editor/micro/v2.0.13/assets/micro-logo-mark.svg | resized to 36 px |
| kakoune | kakoune.svg | https://raw.githubusercontent.com/mawww/kakoune/v2024.05.18/doc/kakoune_logo.svg | resized to 36 px |
| vim | vim.png | https://raw.githubusercontent.com/vim/vim/v9.1.0707/runtime/vim48x48.png | its own 48x48 pixels, unmodified; vim-licence.txt (runtime/doc/uganda.txt at the same tag) beside it |
| bat | bat.svg | https://raw.githubusercontent.com/sharkdp/bat/v0.24.0/doc/logo-header.svg | resized to fit 36 px |
| nnn | nnn.svg | https://raw.githubusercontent.com/jarun/nnn/v4.9/misc/logo/logo.svg | resized to 36 px |
| fzf | fzf.png | https://junegunn.github.io/android-chrome-192x192.png (the site's page links /android-chrome-192x192.png) | resized to 36 px |
| flac | flac.gif | https://xiph.org/flac/images/logo130.gif | resized to fit 36 px |
| lame | lame.gif | https://sourceforge.net/p/lame/svn/HEAD/tree/trunk/lame/doc/html/images/logo.gif?format=raw | resized to fit 36 px |
| profanity | profanity.png | https://raw.githubusercontent.com/profanity-im/profanity/0.14.0/icons/proIcon.png | its own 32x32 pixels |

- lame: the logo file was read from the LAME source tree (SVN trunk,
  doc/html/images/logo.gif, a 358x231 GIF, as the mirrored docs said), so
  the row is confirmed. lame.sourceforge.io still returned 403.
- age: no file is vendored. Its terms allow only unaltered use, and its
  logo.png is 1246x592, which no tile shows at its own pixels, so age keeps
  its NONOS-drawn icon, as the ruling above says.
- No logo (eza, ncdu, tig, lazygit, minisign, geth) and age: the store
  draws a NONOS icon from the toolkit's mask set, tinted, by kind.
- git's credit ("Git Logo by Jason Long, CC BY 3.0"), the ™ and vim's
  licence text are recorded in userland/capsule_market/logos/README.md;
  where the store shows them is UI/UX's (logos-design.md section 3).
