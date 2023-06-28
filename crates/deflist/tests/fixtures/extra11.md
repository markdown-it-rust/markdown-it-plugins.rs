Nested CM syntaxes
......

term
:  A paragraph

    | a | b |
    | - | - |
    | c | d |

    - a
    - b
      - c

    ```rust
    let a = 1;
    ```

......

<dl data-sourcepos="1:1-14:7">
<dt data-sourcepos="1:1-1:4">term</dt>
<dd data-sourcepos="2:4-14:7">
<p data-sourcepos="2:4-2:14">A paragraph</p>
<table data-sourcepos="4:5-6:13">
<thead data-sourcepos="4:5-5:13">
<tr data-sourcepos="4:5-4:13">
<th data-sourcepos="4:7-4:7">a</th>
<th data-sourcepos="4:11-4:11">b</th>
</tr>
</thead>
<tbody data-sourcepos="6:5-6:13">
<tr data-sourcepos="6:5-6:13">
<td data-sourcepos="6:7-6:7">c</td>
<td data-sourcepos="6:11-6:11">d</td>
</tr>
</tbody>
</table>
<ul data-sourcepos="8:5-11:0">
<li data-sourcepos="8:5-8:7">a</li>
<li data-sourcepos="9:5-11:0">b
<ul data-sourcepos="10:7-11:0">
<li data-sourcepos="10:7-11:0">c</li>
</ul>
</li>
</ul>
<pre><code data-sourcepos="12:5-14:7" class="language-rust">let a = 1;
</code></pre>
</dd>
</dl>
