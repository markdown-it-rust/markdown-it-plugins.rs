Regression test (blockquote inside deflist)
......

foo
: > bar
: baz

......

<dl data-sourcepos="1:1-3:5">
<dt data-sourcepos="1:1-1:3">foo</dt>
<dd data-sourcepos="2:3-2:7">
<blockquote data-sourcepos="2:3-2:7">
<p data-sourcepos="2:5-2:7">bar</p>
</blockquote>
</dd>
<dd data-sourcepos="3:3-3:5">baz</dd>
</dl>
