Example 280
(adds classes to ul/li elements)
......

- [x] foo
  - [ ] bar
  - [x] baz
- [ ] bim

......

<ul class="contains-task-list">
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" disabled="" checked="" /> foo
<ul class="contains-task-list">
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" disabled="" /> bar</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" disabled="" checked="" /> baz</li>
</ul>
</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" disabled="" /> bim</li>
</ul>
