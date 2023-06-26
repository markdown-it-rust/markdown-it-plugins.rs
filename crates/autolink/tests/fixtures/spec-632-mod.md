https://github.github.com/gfm/#example-632
., -, and _ can occur on both sides of the @, but only . may occur at the end of the email address, in which case it will not be considered part of the address

THIS IS MODIFIED FROM THE SPEC, to account for the known failure of `_` already being matched as an emphasis marker,
......

a.b-c-d@a.b

a.b-c-d@a.b.

a.b-c-d@a.b-

a.b-c-d@a.b_

......

<p data-sourcepos="1:1-1:11"><a data-sourcepos="1:1-1:11" href="mailto:a.b-c-d@a.b">a.b-c-d@a.b</a></p>
<p data-sourcepos="3:1-3:12"><a data-sourcepos="3:1-3:11" href="mailto:a.b-c-d@a.b">a.b-c-d@a.b</a>.</p>
<p data-sourcepos="5:1-5:12">a.b-c-d@a.b-</p>
<p data-sourcepos="7:1-7:12">a.b-c-d@a.b_</p>
