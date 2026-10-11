# foot's drawing side, for foot.sh: pixman, FreeType, fontconfig on expat
# (lib/foot-wayland.sh builds it), tllist and fcft, and the one font the
# guest is given. Needs lib/meson.sh sourced and meson_setup run first.

fonts_build() {
	meson_target "$(unpack pixman)" -Dtests=disabled -Ddemos=disabled -Dgtk=disabled \
		-Dlibpng=disabled -Dopenmp=disabled
	# Each of these defaults to auto, and auto would take whatever the
	# build host has, or download it.
	meson_target "$(unpack freetype)" -Dzlib=disabled -Dbzip2=disabled -Dpng=disabled \
		-Dharfbuzz=disabled -Dbrotli=disabled
	fc=$(unpack fontconfig)
	meson_target "$fc" -Ddoc=disabled -Dnls=disabled -Dtests=disabled -Dtools=disabled \
		-Dcache-build=disabled -Dxml-backend=expat
	meson_target "$(unpack tllist)"
	# No shaping: foot draws a cell at a time, and either would need
	# HarfBuzz and utf8proc. Its SVG glyphs come from the nanosvg it carries.
	meson_target "$(unpack fcft)" -Ddocs=disabled -Dgrapheme-shaping=disabled \
		-Drun-shaping=disabled
}

# fonts_out OUT: DejaVu Sans Mono in its four faces, the config fontconfig
# reads from /etc/fonts/fonts.conf, and the rule from fontconfig's own
# 60-latin.conf that makes the monospace family DejaVu Sans Mono, which
# foot asks for by default. The cache goes under the guest's home, the one
# place it can write.
fonts_out() {
	dv=$(unpack dejavu-fonts)
	mkdir -p "$1/fonts"
	for face in "" -Bold -Oblique -BoldOblique; do
		cp "$dv/ttf/DejaVuSansMono$face.ttf" "$1/fonts/"
	done
	cp "$fc/conf.d/60-latin.conf" "$1/60-latin.conf"
	cat >"$1/fonts.conf" <<-'EOF'
		<?xml version="1.0"?>
		<!DOCTYPE fontconfig SYSTEM "urn:fontconfig:fonts.dtd">
		<fontconfig>
			<dir>/usr/share/fonts/dejavu</dir>
			<cachedir prefix="xdg">fontconfig</cachedir>
			<include>/usr/share/fontconfig/conf.avail/60-latin.conf</include>
		</fontconfig>
	EOF
}
