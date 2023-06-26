https://github.github.com/gfm/#example-625
When an autolink ends in ), we scan the entire autolink for the total number of parentheses. If there is a greater number of closing parentheses than opening ones, we don’t consider the unmatched trailing parentheses part of the autolink, in order to facilitate including an autolink inside a parenthesis:
......

www.google.com/search?q=Markup+(business)

www.google.com/search?q=Markup+(business)))

(www.google.com/search?q=Markup+(business))

(www.google.com/search?q=Markup+(business)

......

<p data-sourcepos="1:1-1:41"><a data-sourcepos="1:1-1:41" href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a></p>
<p data-sourcepos="3:1-3:43"><a data-sourcepos="3:1-3:41" href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a>))</p>
<p data-sourcepos="5:1-5:43">(<a data-sourcepos="5:2-5:42" href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a>)</p>
<p data-sourcepos="7:1-7:42">(<a data-sourcepos="7:2-7:42" href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a></p>
