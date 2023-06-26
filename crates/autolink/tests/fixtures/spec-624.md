https://github.github.com/gfm/#example-624
Trailing punctuation (specifically, ?, !, ., ,, :, *, _, and ~) will not be considered part of the autolink, though they may be included in the interior of the link:
......

Visit www.commonmark.org.

Visit www.commonmark.org/a.b.

......

<p data-sourcepos="1:1-1:25">Visit <a data-sourcepos="1:7-1:24" href="http://www.commonmark.org">www.commonmark.org</a>.</p>
<p data-sourcepos="3:1-3:29">Visit <a data-sourcepos="3:7-3:28" href="http://www.commonmark.org/a.b">www.commonmark.org/a.b</a>.</p>
