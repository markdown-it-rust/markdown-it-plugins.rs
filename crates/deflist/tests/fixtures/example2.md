A term may have multiple definitions, and each definition may consist of one or more block elements (paragraph, code block, list, etc.), each indented four spaces or one tab stop. The body of the definition (not including the first line) should be indented four spaces. However, as with other Markdown lists, you can “lazily” omit indentation except at the beginning of a paragraph or other block element
......

Term 1

:   Definition
with lazy continuation.

    Second paragraph of the definition.

......

<dl data-sourcepos="1:1-6:39">
<dt data-sourcepos="1:1-1:6">Term 1</dt>
<dd data-sourcepos="3:5-6:39">
<p data-sourcepos="3:5-4:23">Definition
with lazy continuation.</p>
<p data-sourcepos="6:5-6:39">Second paragraph of the definition.</p>
</dd>
</dl>
