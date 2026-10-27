# DejaVu 2.37, the faces a Linux window program draws its text in: Sans,
# Sans Bold, Sans Mono and Serif, with the fonts' licence. Data, never run.
# fontconfig, which the programs bring from their own packages, finds them
# under /usr/share/fonts and makes them sans-serif, monospace and serif.
#   OUT/fonts/DejaVuSans.ttf, DejaVuSans-Bold.ttf, DejaVuSansMono.ttf,
#   DejaVuSerif.ttf, LICENSE
tarball=$(fetch dejavu-fonts)
tar -xjf "$tarball" -C "$work"
d="$work/dejavu-fonts-ttf-2.37"
mkdir -p "$out/fonts"
for face in DejaVuSans DejaVuSans-Bold DejaVuSansMono DejaVuSerif; do
	cp "$d/ttf/$face.ttf" "$out/fonts/$face.ttf"
done
cp "$d/LICENSE" "$out/fonts/LICENSE"
