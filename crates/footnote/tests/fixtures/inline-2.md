Inline footnotes can have arbitrary markup

......

foo^[ *bar* ]

......

<p data-sourcepos="1:1-1:13">foo<sup class="footnote-ref"><a href="#fn1" id="fnref1">[1]</a></sup></p>
<hr class="footnotes-sep">
<section class="footnotes">
<ol class="footnotes-list">
<li id="fn1" class="footnote-item">
<p> <em data-sourcepos="1:7-1:11">bar</em>  <a href="#fnref1" class="footnote-backref">↩︎</a></p>
</li>
</ol>
</section>
