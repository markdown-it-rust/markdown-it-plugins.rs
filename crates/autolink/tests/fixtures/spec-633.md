https://github.github.com/gfm/#example-633
An extended protocol autolink will be recognised when a protocol is recognised within any text node. Valid protocols are `mailto` and `xmpp`
......

mailto:foo@bar.baz

mailto:a.b-c_d@a.b

mailto:a.b-c_d@a.b.

mailto:a.b-c_d@a.b/

mailto:a.b-c_d@a.b-

mailto:a.b-c_d@a.b_

xmpp:foo@bar.baz

xmpp:foo@bar.baz.

......

<p data-sourcepos="1:1-1:18"><a data-sourcepos="1:1-1:18" href="mailto:foo@bar.baz">mailto:foo@bar.baz</a></p>
<p data-sourcepos="3:1-3:18"><a data-sourcepos="3:1-3:18" href="mailto:a.b-c_d@a.b">mailto:a.b-c_d@a.b</a></p>
<p data-sourcepos="5:1-5:19"><a data-sourcepos="5:1-5:18" href="mailto:a.b-c_d@a.b">mailto:a.b-c_d@a.b</a>.</p>
<p data-sourcepos="7:1-7:19"><a data-sourcepos="7:1-7:18" href="mailto:a.b-c_d@a.b">mailto:a.b-c_d@a.b</a>/</p>
<p data-sourcepos="9:1-9:19">mailto:a.b-c_d@a.b-</p>
<p data-sourcepos="11:1-11:19">mailto:a.b-c_d@a.b_</p>
<p data-sourcepos="13:1-13:16"><a data-sourcepos="13:1-13:16" href="xmpp:foo@bar.baz">xmpp:foo@bar.baz</a></p>
<p data-sourcepos="15:1-15:17"><a data-sourcepos="15:1-15:16" href="xmpp:foo@bar.baz">xmpp:foo@bar.baz</a>.</p>
