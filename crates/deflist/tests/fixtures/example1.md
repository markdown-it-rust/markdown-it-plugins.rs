Each term must fit on one line, which may optionally be followed by a blank line, and must be followed by one or more definitions. A definition begins with a colon or tilde, which may be indented one or two spaces.
......

Term 1

: Definition 1

Term 2 with *inline markup*

: Definition 2

      { some code, part of Definition 2 }

  Third paragraph of definition 2.

......

<dl data-sourcepos="1:1-11:34">
<dt data-sourcepos="1:1-1:6">Term 1</dt>
<dd data-sourcepos="3:3-4:0">
<p data-sourcepos="3:3-3:14">Definition 1</p>
</dd>
<dt data-sourcepos="5:1-5:27">Term 2 with <em data-sourcepos="5:13-5:27">inline markup</em></dt>
<dd data-sourcepos="5:1-11:34">
<p data-sourcepos="7:3-7:14">Definition 2</p>
<pre><code data-sourcepos="9:7-9:41">{ some code, part of Definition 2 }
</code></pre>
<p data-sourcepos="11:3-11:34">Third paragraph of definition 2.</p>
</dd>
</dl>
