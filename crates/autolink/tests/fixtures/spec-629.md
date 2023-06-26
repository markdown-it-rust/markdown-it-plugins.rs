https://github.github.com/gfm/#example-629
An extended url autolink will be recognised when one of the schemes http://, or https://, followed by a valid domain, then zero or more non-space non-< characters according to extended autolink path validation:
......

http://commonmark.org

(Visit https://encrypted.google.com/search?q=Markup+(business))

......

<p data-sourcepos="1:1-1:21"><a data-sourcepos="1:1-1:21" href="http://commonmark.org">http://commonmark.org</a></p>
<p data-sourcepos="3:1-3:63">(Visit <a data-sourcepos="3:8-3:62" href="https://encrypted.google.com/search?q=Markup+(business)">https://encrypted.google.com/search?q=Markup+(business)</a>)</p>
