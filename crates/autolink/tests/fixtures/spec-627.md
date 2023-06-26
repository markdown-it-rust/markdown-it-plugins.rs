https://github.github.com/gfm/#example-627
If an autolink ends in a semicolon (;), we check to see if it appears to resemble an entity reference; if the preceding text is & followed by one or more alphanumeric characters. If so, it is excluded from the autolink:
......

www.google.com/search?q=commonmark&hl=en

www.google.com/search?q=commonmark&hl;

......

<p data-sourcepos="1:1-1:40"><a data-sourcepos="1:1-1:40" href="http://www.google.com/search?q=commonmark&amp;hl=en">www.google.com/search?q=commonmark&amp;hl=en</a></p>
<p data-sourcepos="3:1-3:38"><a data-sourcepos="3:1-3:34" href="http://www.google.com/search?q=commonmark">www.google.com/search?q=commonmark</a>&amp;hl;</p>
