Nested definition lists
......

test
  : foo
      : bar
          : baz
      : bar2
  : foo2

......

<dl data-sourcepos="1:1-6:8">
<dt data-sourcepos="1:1-1:4">test</dt>
<dd data-sourcepos="2:5-5:12">
<dl data-sourcepos="2:5-5:12">
<dt data-sourcepos="2:5-2:7">foo</dt>
<dd data-sourcepos="3:9-4:15">
<dl data-sourcepos="3:9-4:15">
<dt data-sourcepos="3:9-3:11">bar</dt>
<dd data-sourcepos="4:13-4:15">baz</dd>
</dl>
</dd>
<dd data-sourcepos="5:9-5:12">bar2</dd>
</dl>
</dd>
<dd data-sourcepos="6:5-6:8">foo2</dd>
</dl>
