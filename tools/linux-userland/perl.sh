# Perl 5.44.0, static: the perl interpreter with its core XS modules compiled
# in, cross-built with perl-cross 1.6.5, and a trimmed pure-Perl library as
# data beside it.
#   OUT/perl, and its library under OUT/perl5, for /usr/lib/perl5
# The Perl hash pinned is the one CPAN publishes beside the tarball
# (perl-5.44.0.tar.gz.sha256.txt); perl-cross's is the one its release lists
# (perl-cross-1.6.5.hash).
tar -xzf "$(fetch perl)" -C "$work"
tar -xzf "$(fetch perl-cross)" -C "$work"
p="$work/perl-5.44.0"
# perl-cross's configure replaces perl's Configure, which nothing here runs.
# On a file system that ignores case (a Mac's) the two are one file, and
# perl ships Configure read-only, so the copy below was refused there.
rm -f "$p/Configure"
cp -R "$work/perl-cross-1.6.5/." "$p/"
# perl-cross names each target tool by a prefix; zig plays the compiler and
# archiver, and LLVM's nm, objdump and readelf read the x86-64 objects. The
# build machine's own may not: a Mac's nm reads only Mach-O and it has no
# readelf, and the seal on a Mac stopped here.
mkdir -p "$work/zig"
t="$work/zig/x86_64-linux-musl-"
printf '#!/bin/sh\nexec "%s" cc -target x86_64-linux-musl "$@"\n' "$zig" >"${t}gcc"
printf '#!/bin/sh\nexec "%s" ar "$@"\n' "$zig" >"${t}ar"
printf '#!/bin/sh\nexec "%s" ranlib "$@"\n' "$zig" >"${t}ranlib"
for tool in nm objdump readelf; do
	command -v "llvm-$tool" >/dev/null || { echo "nonos-linux-userland-build: perl needs llvm-$tool" >&3; exit 1; }
	printf '#!/bin/sh\nexec "%s" "$@"\n' "$(command -v "llvm-$tool")" >"${t}$tool"
done
chmod +x "$t"*
# perl-cross also builds a miniperl for the build machine, and finds that
# machine's type sizes and byte order by reading its compiler's objects as
# ELF. A Mac's objects are Mach-O, so there they are measured by running a
# program built for it, and given to that build as settings. perl-cross has
# hints for Linux and none for a Mac, and nanosleep is set only by hints, so
# its config.h line came out empty; the same program uses nanosleep, so it
# builds only where nanosleep is there, and says so.
hostset=""
if [ "$(uname -s)" != Linux ]; then
	cat >"$work/sizes.c" <<'EOF'
#include <stdio.h>
#include <sys/types.h>
#include <time.h>
#include <unistd.h>
#define Z(n, t) printf(n " %d\n", (int)sizeof(t))
int main(void) {
	unsigned long long u = 0x0807060504030201ULL;
	const unsigned char *b = (const unsigned char *)&u;
	struct timespec ts = { 0, 0 };
	int i;
	if (nanosleep(&ts, 0) != 0)
		return 1;
	printf("d_nanosleep define\n");
	Z("charsize", char); Z("shortsize", short); Z("intsize", int);
	Z("longsize", long); Z("doublesize", double); Z("ptrsize", void *);
	Z("longdblsize", long double); Z("longlongsize", long long);
	Z("sizesize", size_t); Z("fpossize", fpos_t); Z("lseeksize", off_t);
	Z("uidsize", uid_t); Z("gidsize", gid_t); Z("timesize", time_t);
	printf("byteorder ");
	for (i = 0; i < (int)sizeof u; i++)
		printf("%d", b[i]);
	printf("\n");
	return 0;
}
EOF
	cc -o "$work/sizes" "$work/sizes.c" >&2
	"$work/sizes" >"$work/sizes.out" || { echo "nonos-linux-userland-build: perl cannot measure the build machine" >&3; exit 1; }
	hostset=$(while read -r k v; do printf ' --host-set-%s=%s=%s' "$k" "$k" "$v"; done <"$work/sizes.out")
	# perl-cross names the build machine's system from its compiler's target
	# and knows no Mac, so the miniperl's $^O was empty, and MakeMaker, which
	# picks its rules by $^O, failed at the first module (make dynaloader).
	# It is given a name of its own: any name but darwin makes it a plain
	# Unix to MakeMaker, and darwin would put Mac-only code into the
	# DynaLoader.pm and XSLoader.pm this Linux perl ships.
	# Linux's hints are also what link libm; a Mac's libm is part of its
	# system library and links as -lm too.
	hostset="$hostset --host-set-osname=osname=buildhost --host-libs=libs=m"
	# The flags perl's own build compiles with on a Mac (hints/darwin.sh),
	# and the two its code needs from clang: perl-cross has no Mac hints and
	# compiled the miniperl with none, and it panicked as it first ran.
	# perl-cross takes the build machine's compiler from HOSTCC.
	printf '#!/bin/sh\nexec cc -fno-common -DPERL_DARWIN -fwrapv -fno-strict-aliasing "$@"\n' >"$work/hostcc"
	chmod +x "$work/hostcc"
	HOSTCC="$work/hostcc"
	export HOSTCC
	echo "perl: the build machine's miniperl takes:$hostset" >&2
fi
# The library lives at /usr/lib/perl5, where the store puts it. What perl's
# own Configure would do and perl-cross leaves out:
# - _GNU_SOURCE, under which musl declares the GNU functions perl finds
#   (memrchr);
# - -fwrapv and -fno-strict-aliasing, which perl's code needs; without them
#   clang miscompiles the interpreter;
# - the preprocessor: zig takes a file read from standard input as C only
#   when told, and otherwise reads the build machine's headers, so Errno.pm
#   would list another libc's error numbers. perl-cross also passes -P, which
#   drops the line markers Errno.pm is found by; clang, unlike gcc 5 and
#   later, keeps each line whole without it.
# Perl is built without locale support (NO_LOCALE): the personality offers
# only the C locale, and musl names a mixed locale in a form perl-cross does
# not probe for, which stops perl as it starts. No dynamic loading: every XS
# module is linked into the one binary, since the personality maps no file
# it has not proved. Left out are the XS test modules and re's XS half, which
# carries its own copy of the regex compiler and links only as a loadable
# module, so the re pragma is not carried, nor Text::Wrap, which uses it.
# perl-cross takes a target tool from the environment before its prefix
# (READELF, as the build environment names its own), and keeps an
# environment value for any setting it would leave empty (src, the source's
# store path). Neither belongs to this build, so neither is passed on.
(unset src READELF OBJDUMP NM && cd "$p" &&
	# shellcheck disable=SC2086 # each setting is one word
	./configure --target=x86_64-linux-musl --target-tools-prefix="$t" --prefix=/usr $hostset \
		--all-static --disable-mod=ext/re,ext/XS-APItest,ext/XS-Typemap -Accflags="-D_GNU_SOURCE -DNO_LOCALE -fwrapv -fno-strict-aliasing" -Dprivlib=/usr/lib/perl5 -Darchlib=/usr/lib/perl5 \
		-Dsitelib=/usr/lib/perl5/site -Dsitearch=/usr/lib/perl5/site \
		-Dvendorlib=/usr/lib/perl5/vendor -Dvendorarch=/usr/lib/perl5/vendor \
		-Doptimize="-O2 -g0 -fno-pie -ffile-prefix-map=$work=/build" \
		-Dcpp="${t}gcc -x c -E" -Dcppstdin="${t}gcc -x c -E" -Dcpprun="${t}gcc -x c -E" \
		-Dldflags="-static -no-pie -s" -Dcf_by=nonos -Dcf_email=nonos@localhost \
		-Dmyhostname=nonos -Dperladmin=root -Dcf_time="Wed Sep 30 00:00:00 UTC 2026" >&2 &&
	# perl -V names the day it was compiled; zig refuses __DATE__ and
	# __TIME__, so it names the release day, 30 September 2026.
	echo '#define PERL_BUILD_DATE "Sep 30 2026 00:00:00"' >>config.h &&
	make -j8 >&2 || {
		# A parallel make reports the step that failed far above its last
		# lines, so the lines naming it are shown first.
		grep -n -E '\*\*\*|rror|No such|not found|Killed' "$log" | grep -v Werror | head -40 >&3
		exit 1
	})
cp "$p/perl" "$out/perl"
# The library, as userland/linux_userland/perl-lib.txt lists it.
# A module's file is in lib/ once the build has made it, and an XS module
# linked in statically leaves its own in its directory under ext/, dist/ or
# cpan/, which is named after the module or its distribution
# (ext/File-Glob/Glob.pm, cpan/Time-Piece/Seconds.pm). Each file found must
# declare the package it is taken for.
pm() {
	[ -f "$p/lib/$1.pm" ] && { echo "$p/lib/$1.pm"; return; }
	top=${1%%/*}
	for at in "*/blib/lib/$1.pm" "*/lib/$1.pm" "*/$1.pm" "*/$(echo "$1" | tr / -)/${1##*/}.pm" \
		"*/$top-*/${1##*/}.pm" "*/$top/*/${1##*/}.pm"; do
		f=$(find "$p/ext" "$p/dist" "$p/cpan" -path "$at" -not -path '*/t/*' | sort | head -1)
		[ -n "$f" ] && { echo "$f"; return; }
	done
	echo "nonos-linux-userland-build: perl has no $1.pm" >&3
	exit 1
}
mkdir -p "$out/perl5"
for f in $(grep -v '^#' "$root/userland/linux_userland/perl-lib.txt"); do
	mkdir -p "$out/perl5/$(dirname "$f")"
	case $f in
	*.pm)
		m=${f%.pm}
		src=$(pm "$m")
		if ! grep -q "^package $(echo "$m" | sed 's|/|::|g')\b" "$src"; then
			echo "nonos-linux-userland-build: $src is not $m" >&3
			exit 1
		fi
		;;
	*) src="$p/lib/$f" ;;
	esac
	cp "$src" "$out/perl5/$f"
done
# Config records the tools perl was built with, which live in this build's
# work directory; it is mapped to /build, as the compiler maps it in the
# binary, so two builds give the same files. It also records the configure
# command, which on a Mac carried the build machine's settings above; they
# describe only the miniperl, so they are left out, and a Mac and a Linux
# machine give the same files.
taken=$(echo $hostset | wc -w)
for f in Config.pm Config_heavy.pl; do
	argc=$(sed -n "s/^config_argc='\([0-9]*\)'\$/\1/p" "$out/perl5/$f")
	sed -e "s|$work|/build|g" -e "s| --host-[a-z_-]*=[a-z_]*=[a-z0-9]*||g" \
		${argc:+-e "s/^config_argc='$argc'\$/config_argc='$((argc - taken))'/"} \
		"$out/perl5/$f" >"$work/$f" && cp "$work/$f" "$out/perl5/$f"
done
# Every module above loads from the trimmed library alone: perl, a static
# x86-64 Linux program, loads each one there, when the build machine can run
# it, and the build stops if one needs a file left out.
if [ "$(uname -s)-$(uname -m)" = Linux-x86_64 ]; then
	for f in $(cd "$out/perl5" && find . -name '*.pm' | sed 's|^\./||'); do
		env -i PERL5LIB="$out/perl5" "$out/perl" -e 'require $ARGV[0]; for (values %INC) { die "$_ is not in the library\n" unless index($_, $ARGV[1]) == 0 }' \
			"$f" "$out/perl5" >&2 || { echo "nonos-linux-userland-build: perl cannot load $f from its library" >&3; exit 1; }
	done
fi
