// NONOS Operating System (AGPL-3.0-or-later)
// location said http://localhost/ no matter what had been fetched, so a page
// that reads its own address was told something that was never true.

export function locationChecks(ok) {
  const at = u => globalThis.__njs_mkloc(u);

  const l = at('https://example.org:8443/docs/intro?q=rust#top');
  ok(l.protocol === 'https:', `protocol: ${l.protocol}`);
  ok(l.host === 'example.org:8443', `host keeps the port: ${l.host}`);
  ok(l.hostname === 'example.org', `hostname drops it: ${l.hostname}`);
  ok(l.port === '8443', `port: ${l.port}`);
  ok(l.pathname === '/docs/intro', `pathname: ${l.pathname}`);
  ok(l.search === '?q=rust', `search: ${l.search}`);
  ok(l.hash === '#top', `hash: ${l.hash}`);
  ok(l.origin === 'https://example.org:8443', `origin: ${l.origin}`);
  ok(String(l) === l.href, 'it stringifies to its href');

  // No port is the common case, and a hostname that kept a colon would not
  // match anything a page compares it against.
  const plain = at('http://example.org/');
  ok(plain.hostname === 'example.org', `no port leaves the hostname alone: ${plain.hostname}`);
  ok(plain.port === '', 'no port reports empty');
  ok(plain.search === '' && plain.hash === '', 'absent parts are empty, not undefined');

  // A bare host with no trailing slash still has a path, because code that
  // reads pathname and indexes into it would otherwise throw.
  const bare = at('http://example.org');
  ok(bare.pathname === '/', `a missing path is the root: ${bare.pathname}`);

  // An address with a colon in the path must not be read as a port.
  const colon = at('http://example.org/a:b');
  ok(colon.hostname === 'example.org', `a colon in the path is not a port: ${colon.hostname}`);

  // Writing to location goes somewhere: href, location itself, pathname and
  // search navigate; a new hash moves within the page and says so.
  const went = [];
  globalThis.__njs_navigate = h => went.push(h);
  globalThis.__njs_setloc(at('https://example.org/a/b?x=1#top'));
  globalThis.location.href = '/next';
  globalThis.location = 'https://other.example/';
  globalThis.location.pathname = 'p';
  globalThis.location.search = 'q=2';
  ok(went.join(' ') === '/next https://other.example/ https://example.org/p?x=1#top '
     + 'https://example.org/a/b?q=2#top', `each write navigates: ${went.join(' ')}`);
  ok(globalThis.location.href === 'https://example.org/a/b?x=1#top', 'and nothing moved yet');

  let changed = null;
  globalThis.addEventListener('hashchange', ev => { changed = ev; });
  globalThis.location.hash = 'list';
  ok(globalThis.location.hash === '#list', `the hash moved: ${globalThis.location.hash}`);
  ok(globalThis.location.href === 'https://example.org/a/b?x=1#list', 'and the address with it');
  ok(changed && changed.oldURL.endsWith('#top') && changed.newURL.endsWith('#list'),
     'hashchange says from where to where');
  ok(went.length === 4, 'a hash is no navigation');
  changed = null;
  globalThis.location.hash = '#list';
  ok(changed === null, 'the same hash again changes nothing');
}
