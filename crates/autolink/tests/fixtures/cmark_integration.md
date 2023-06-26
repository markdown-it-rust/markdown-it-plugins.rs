Integration with other Commonmark syntax
......

# Heading http://www.example.com

[http://example.com](http://example.com)
[mailto:bob@example.com](https://example.com)
[bob@example.com](https://example.com)

<http://www.example.com>www.example.com

**www.example.com**

*https://example.com*

......

<h1 data-sourcepos="1:1-1:32">Heading <a data-sourcepos="1:11-1:32" href="http://www.example.com">http://www.example.com</a></h1>
<p data-sourcepos="3:1-5:38"><a data-sourcepos="3:1-3:40" href="http://example.com">http://example.com</a>
<a data-sourcepos="4:1-4:45" href="https://example.com">mailto:bob@example.com</a>
<a data-sourcepos="5:1-5:38" href="https://example.com">bob@example.com</a></p>
<p data-sourcepos="7:1-7:39"><a data-sourcepos="7:1-7:24" href="http://www.example.com">http://www.example.com</a>www.example.com</p>
<p data-sourcepos="9:1-9:19"><strong data-sourcepos="9:1-9:19"><a data-sourcepos="9:3-9:17" href="http://www.example.com">www.example.com</a></strong></p>
<p data-sourcepos="11:1-11:21"><em data-sourcepos="11:1-11:21"><a data-sourcepos="11:2-11:20" href="https://example.com">https://example.com</a></em></p>
