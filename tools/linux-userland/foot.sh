# foot 1.28.0, the Wayland terminal, static, on libwayland 1.26.0 with
# wayland-protocols 1.49, libxkbcommon 1.7.0, pixman 0.46.4, FreeType
# 2.14.3, fontconfig 2.18.3 on expat 2.9.0, tllist 1.1.0 and fcft 3.3.3, with
# DejaVu Sans Mono 2.37 as its one font. Built by meson 1.12.1.
#   OUT/foot, OUT/fonts/DejaVuSansMono*.ttf, OUT/fonts.conf, OUT/60-latin.conf
#
# Not yet built: this recipe has had no run. Its first is the owner's.
#
# Each pin's sha256, as the maintainers checked it, matches its publisher's own
# statement: a .sha256sum or sha512 file, the release announcement's hash,
# or, where a signature is all a project publishes (expat, tllist, fcft,
# foot), a good signature from its author's key. libxkbcommon stays at
# 1.7.0, the last release with a published tarball and checksum; foot
# needs 1.6.0 or later.
#
# From the build host, as pkg-config already is for the other recipes:
# python3 3.10 or later (meson, and the generators in foot, fcft,
# fontconfig, FreeType and wayland), ninja, pkg-config, gperf (fontconfig),
# bison or byacc (libxkbcommon), and the host's C compiler for the host
# copies of expat and wayland-scanner.
#
# foot is built without its terminfo: its TERM is xterm-256color, which the
# guest's terminal programs carry compiled in (lib/terminal.sh, the pinned
# ncurses' own entry), so no terminfo file or host tic is needed. No IME,
# no grapheme clustering (so no utf8proc), no docs, no utmp helper.
for tool in python3 ninja pkg-config gperf; do
	command -v "$tool" >/dev/null || { echo "foot: the build host has no $tool" >&3; exit 1; }
done
command -v bison >/dev/null || command -v byacc >/dev/null ||
	{ echo "foot: the build host has neither bison nor byacc" >&3; exit 1; }
jobs="${NIX_BUILD_CORES:-0}"
[ "$jobs" -gt 0 ] 2>/dev/null || jobs="$(nproc 2>/dev/null || sysctl -n hw.logicalcpu 2>/dev/null || echo 4)"
deps="$work/deps"
hostdeps="$work/hostdeps"
mkdir -p "$deps" "$hostdeps"
. "$root/tools/linux-userland/lib/meson.sh"
. "$root/tools/linux-userland/lib/foot-wayland.sh"
. "$root/tools/linux-userland/lib/foot-fonts.sh"
meson_setup
wayland_build
fonts_build
# werror is foot's default, and zig's clang warns where foot's own CI
# compilers do not.
ft=$(unpack foot)
$meson setup "$ft.target" "$ft" --cross-file "$work/cross.ini" --native-file "$work/native.ini" \
	--prefix=/usr --buildtype=plain --default-library=static --prefer-static \
	--wrap-mode=nodownload -Db_pie=false -Dwerror=false -Ddocs=disabled -Dtests=false \
	-Dime=false -Dgrapheme-clustering=disabled -Dterminfo=disabled \
	-Ddefault-terminfo=xterm-256color -Dutmp-backend=none >/dev/null
ninja -C "$ft.target" -j"$jobs" foot >/dev/null
cp "$ft.target/foot" "$out/foot"
fonts_out "$out"
