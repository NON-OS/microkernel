/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * Prelude text, first part: timers, console, navigator.
 */

    "(function(){var q=[];"
    "globalThis.window=globalThis;globalThis.self=globalThis;"
    "var sink=function(){};"
    "globalThis.console={log:sink,warn:sink,error:sink,info:sink,debug:sink,trace:sink};"
    /* Timers keep their delay and fire in due order. Dropping the delay ran
     * a chain of deferred steps out of sequence, which is worse than not
     * running it. Cancelling has to work too: a component that unmounts and
     * clears its timer would otherwise still fire into a tree it gave up. */
    "var tid=1,clock=0,cur=0;"
    "globalThis.setTimeout=function(f,ms){q.push({id:tid,at:clock+(+ms||0),f:f,r:0});return tid++;};"
    "globalThis.setInterval=function(f,ms){var d=Math.max(1,+ms||1);"
    "q.push({id:tid,at:clock+d,f:f,r:d});return tid++;};"
    "globalThis.clearTimeout=function(i){for(var k=0;k<q.length;k++)"
    "if(q[k].id===i){q.splice(k,1);return;}};"
    "globalThis.clearInterval=globalThis.clearTimeout;"
    "globalThis.requestAnimationFrame=function(f){q.push({id:tid,at:clock,f:f,r:0});return tid++;};"
    "globalThis.cancelAnimationFrame=globalThis.clearTimeout;"
    "globalThis.queueMicrotask=globalThis.queueMicrotask||function(f){Promise.resolve().then(f);};"
    /* Due timers first, then the clock moves to the next one waiting. A
     * repeating timer is requeued rather than dropped, and the run count is
     * capped so a timer that reschedules itself cannot hold the pump. */
    /* Run what the clock says is due, and nothing else. The caller passes
     * real elapsed milliseconds, so a repeating timer requeued during this
     * flush lands in the future and waits there. Advancing the clock to meet
     * the earliest timer instead would make every requeue instantly due, and
     * a single setInterval would then run its callback until the iteration
     * cap on every tick, which is a page that never stops working long
     * enough to draw. */
    /* A callback the time budget stopped never reaches the line after it,
     * so `cur` still names it on the next flush. An interval that loops
     * forever is dropped there rather than requeued to freeze every tick. */
    "globalThis.__njs_flush_timers=function(now){var n=0;"
    "if(cur){globalThis.clearTimeout(cur);cur=0;}"
    "if(now>clock)clock=+now;"
    "while(q.length&&n<1000){"
    "var i=0;for(var k=1;k<q.length;k++)if(q[k].at<q[i].at)i=k;"
    "var t=q[i];if(t.at>clock)break;q.splice(i,1);n++;"
    "if(t.r){q.push({id:t.id,at:clock+t.r,f:t.f,r:t.r});}"
    "cur=t.id;try{t.f();}catch(e){"
    "(globalThis.__njs_errors=globalThis.__njs_errors||[]).push(String(e)+' @ '+String(e&&e.stack||''));}"
    "cur=0;}"
    "return n;};"
    /* The same User-Agent every request sends (http/request.rs in the
     * browser), so a page that reads it agrees with what the server saw. */
    "globalThis.navigator={userAgent:'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:128.0) "
    "Gecko/20100101 Firefox/128.0',appVersion:'5.0 (Windows)',platform:'Win32',"
    "vendor:'',product:'Gecko',language:'en-US',languages:['en-US','en'],cookieEnabled:true};"
