nested inline footnotes

......

[Example^[this is another example [a]]][a]

[a]: https://github.com

......

<p data-sourcepos="1:1-1:42"><a data-sourcepos="1:1-1:42" href="https://github.com">Example<sup class="footnote-ref"><a href="#fn1" id="fnref1">[1]</a></sup></a></p>
<hr class="footnotes-sep">
<section class="footnotes">
<ol class="footnotes-list">
<li id="fn1" class="footnote-item">
<p>this is another example <a data-sourcepos="1:35-1:37" href="https://github.com">a</a> <a href="#fnref1" class="footnote-backref">↩︎</a></p>
</li>
</ol>
</section>
