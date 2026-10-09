// NONOS Operating System (AGPL-3.0-or-later)
// window.scrollTo, scroll and scrollBy were not defined, so a "back to top"
// button threw. Each now hands the host a y to move the page to, read from
// either spelling a page uses, and a call with no y moves nothing.

export function scrollChecks(ok) {
  let y = 100;
  const asked = [];
  globalThis.__njs_scroll_y = () => y;
  globalThis.__njs_scroll_to = n => { asked.push(n); y = Math.max(0, n); return y; };

  globalThis.scrollTo(0, 40);
  globalThis.scroll({ top: 70.6, left: 3, behavior: 'smooth' });
  globalThis.scrollBy(0, 30);
  globalThis.scrollBy({ top: -1000 });
  globalThis.scrollTo(0, 'x');
  ok(asked.join() === '40,71,101,-899,0', `each call reaches the host: ${asked.join()}`);
  ok(globalThis.scrollY === 0, 'and scrollY reads where the host put the page');

  globalThis.scrollTo(5);
  globalThis.scrollTo({ left: 9 });
  globalThis.scrollBy({ left: 9 });
  globalThis.scrollTo();
  ok(asked.length === 5, 'a call with no y moves nothing');
}
