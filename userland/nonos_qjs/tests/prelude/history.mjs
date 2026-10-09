// NONOS Operating System (AGPL-3.0-or-later)
// A page that pushes state is telling the reader it moved, so the address
// has to follow. Otherwise the next relative link resolves against the old
// one and code reading the path after a route change sees the previous one.

export function historyChecks(ok) {
  globalThis.__njs_resolve = rel =>
    rel.startsWith('http') ? rel : `https://example.org${rel}`;
  globalThis.__njs_setloc(globalThis.__njs_mkloc('https://example.org/start'));

  globalThis.history.pushState({ page: 1 }, '', '/next');
  ok(globalThis.location.pathname === '/next',
     `pushState moves the address: ${globalThis.location.pathname}`);
  ok(globalThis.history.state.page === 1, 'the state is kept');

  globalThis.history.replaceState({ page: 2 }, '', '/other');
  ok(globalThis.location.pathname === '/other', 'replaceState moves it too');
  ok(globalThis.history.state.page === 2, 'and replaces the state');

  // A push with no address is a state change only, and moving the address
  // would send the next relative link somewhere the page never went.
  const before = globalThis.location.href;
  globalThis.history.pushState({ page: 3 }, '');
  ok(globalThis.location.href === before, 'a push without an address stays put');
  ok(globalThis.history.state.page === 3, 'but still records the state');

  // length counts the browser's entries up to the one shown, then each
  // push made since (two above): seven kept with the third shown is 3 + 2.
  globalThis.__njs_history = k => (k === 0 ? 7 : 2);
  ok(globalThis.history.length === 5, `length: ${globalThis.history.length}`);

  // back, forward and go hand the browser a step through the reader's
  // history; they used to do nothing, so a page's own Back link was dead.
  const steps = [];
  globalThis.__njs_history_go = n => steps.push(n);
  globalThis.history.back();
  globalThis.history.forward();
  globalThis.history.go(-2);
  globalThis.history.go('3');
  ok(steps.join() === '-1,1,-2,3', `each step reaches the browser: ${steps.join()}`);
  let reloaded = 0;
  globalThis.location.reload = () => { reloaded += 1; };
  globalThis.history.go();
  globalThis.history.go(0);
  ok(reloaded === 2 && steps.length === 4, 'go() and go(0) load the page again');
}
