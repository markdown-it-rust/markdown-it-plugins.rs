Regression test (first paragraphs shouldn't be tight):
......

Term 1
: foo

  bar
Term 2
: foo

......

<dl data-sourcepos="1:1-6:5">
<dt data-sourcepos="1:1-1:6">Term 1</dt>
<dd data-sourcepos="2:3-5:6">
<p data-sourcepos="2:3-2:5">foo</p>
<p data-sourcepos="4:3-5:6">bar
Term 2</p>
</dd>
<dd data-sourcepos="6:3-6:5">
<p data-sourcepos="6:3-6:5">foo</p>
</dd>
</dl>
