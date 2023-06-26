Test delimiters:
......

http://google.com/he<lp
mailto:bob@google.co<m
bob@google.co<m

http://google.com/(business)
http://google.com/(business))

http://google.com/other_

......

<p data-sourcepos="1:1-3:15"><a data-sourcepos="1:1-1:20" href="http://google.com/he">http://google.com/he</a>&lt;lp
<a data-sourcepos="2:1-2:20" href="mailto:bob@google.co">mailto:bob@google.co</a>&lt;m
<a data-sourcepos="3:1-3:13" href="mailto:bob@google.co">bob@google.co</a>&lt;m</p>
<p data-sourcepos="5:1-6:29"><a data-sourcepos="5:1-5:28" href="http://google.com/(business)">http://google.com/(business)</a>
<a data-sourcepos="6:1-6:28" href="http://google.com/(business)">http://google.com/(business)</a>)</p>
<p data-sourcepos="8:1-8:24"><a data-sourcepos="8:1-8:23" href="http://google.com/other">http://google.com/other</a>_</p>
