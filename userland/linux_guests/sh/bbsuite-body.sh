# The applets bbsuite runs, one section a line of output or more. A line
# that names a time or a pid starts with @ and is left out of the compare.
W=${BBSUITE_DIR:-/tmp/bbsuite}
rm -rf "$W"; mkdir -p "$W/a/b/c" && cd "$W" || exit 1
echo "mkdir=$?"
printf 'one\ntwo\nthree\ntwo\none\n' > words
printf 'alpha beta\ngamma delta\n' > a/b/text
echo nested > a/b/c/deep
echo "== cat"; cat words a/b/text
echo "== wc"; wc -l words; wc -w a/b/text; wc -c < words
echo "== head tail"; head -n 2 words; tail -n 1 words; tail -c 4 words
echo "== sort uniq"; sort words | uniq -c; sort -r words | head -n 1; sort -u words
echo "== grep"; grep -n two words; grep -rl nested . | sort; grep -c o words; grep -v o words; echo "grep-miss=$(grep -q zzz words; echo $?)"
echo "== sed"; sed 's/o/0/g' words; sed -n '2p' words; sed '/two/d' words
echo "== awk"; awk '{n += length($0)} END {print NR, n}' words; awk -F' ' '{print $2}' a/b/text
echo "== cp mv rm"; cp words copy; cp -r a acopy; mv copy moved; ls; rm moved; rm -r acopy; ls
echo "== find"; find . -type f | sort; find . -name deep; find . -type d | sort
echo "== ls"; ls -la a/b | awk 'NR>1 {print $1, $3, $4, $NF}'; ls -l words | awk '{print $1, $2, $3, $4, $5, $NF}'
echo "@ls-full $(ls -la | tr '\n' '|')"
echo "== touch stat"; touch new; stat -c '%s %F %a %n' new words; test -e new && echo touched
echo "== ln readlink"; ln -s words lnk; readlink lnk; cat lnk | wc -l; ls -l lnk | awk '{print $1, $(NF-2), $(NF-1), $NF}'
echo "== chmod"; chmod 600 words; stat -c '%a' words; chmod u+x,g+r words; stat -c '%a' words; test -x words && echo exec
echo "== test expr"; test 3 -gt 2 && echo gt; [ -d a ] && echo dir; [ -f a ] || echo notfile; expr 6 \* 7; expr length hello; expr 7 % 3
echo "== env"; env -i A=1 B=two env | sort; X=inline sh -c 'echo $X'
echo "== xargs"; printf 'a/b/text\na/b/c/deep\n' | xargs cat; echo 1 2 3 | xargs -n 1 echo n
echo "== dd od"; dd if=/dev/zero bs=512 count=4 2>/dev/null | wc -c; printf 'abc\n' | od -An -tx1; dd if=words bs=1 skip=4 count=3 2>/dev/null; echo
echo "== sha256"; sha256sum words a/b/text
echo "== gzip"; gzip -c words > words.gz; gunzip -c words.gz | sha256sum; cp words w2; gzip w2; ls w2*; gunzip w2.gz; cmp w2 words && echo same
echo "== tar"; tar cf t.tar a; tar tf t.tar | sort; mkdir out; tar xf t.tar -C out; find out -type f | sort; cat out/a/b/c/deep
echo "== pipes redirects"; echo out1 > r; echo out2 >> r; cat < r; ls nothere 2> err; echo "rc=$?"; wc -l < err; { echo g1; echo g2; } | tail -n 1; echo e 2>&1 1>/dev/null | wc -c
echo "== subshell"; (cd a && pwd | sed "s|$W||"); pwd | sed "s|$W|W|"; v=outer; (v=inner; echo $v); echo $v; echo "$(echo sub $(echo nest))"
echo "== bg wait"; (echo bg1 > f1) & (echo bg2 > f2) & wait; cat f1 f2
echo "== trap"; sh -c 'trap "echo exit-trap" EXIT; echo body'; sh -c 'trap "echo got-term" TERM; kill -TERM $$; echo after-term'
echo "== ps"; ps -o pid,comm | awk -v p=$$ '$1 == p {print "self", $2}'; echo "@ps $(ps | wc -l)"
echo "== df du"; df . > /dev/null; echo "df=$?"; echo "@df $(df . | tail -n 1)"; du -s a > /dev/null; echo "du=$?"; echo "@du $(du -s a)"
echo "== date"; echo "@date $(date)"
echo "== dev"; head -c 8 /dev/urandom | wc -c; cat /dev/null | wc -c; echo x > /dev/null; echo "null=$?"
cd / && rm -rf "$W"; echo "cleaned=$?"
