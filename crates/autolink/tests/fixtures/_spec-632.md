https://github.github.com/gfm/#example-632
., -, and _ can occur on both sides of the @, but only . may occur at the end of the email address, in which case it will not be considered part of the address:

THIS IS KNOWN TO FAIL, since `_` is already matched as an emphasis marker.
......

a.b-c_d@a.b

a.b-c_d@a.b.

a.b-c_d@a.b-

a.b-c_d@a.b_

......

<p data-sourcepos="1:1-1:11"><a data-sourcepos="1:7-1:11" href="mailto:a.bc_d@a.b">d@a.b</a></p>
<p data-sourcepos="3:1-3:12"><a data-sourcepos="3:7-3:11" href="mailto:a.b-c_d@a.b">d@a.b</a>.</p>
<p data-sourcepos="5:1-5:12">a.b-c_d@a.b-</p>
<p data-sourcepos="7:1-7:12">a.b-c_d@a.b_</p>
