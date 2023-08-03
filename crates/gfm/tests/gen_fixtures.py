"""Simple script to generate the separate test fixtures from the full spec.txt file.

From: https://github.com/github/cmark-gfm/blob/master/test/spec.txt

Note: https://github.com/github/cmark-gfm/issues/288
"""
from pathlib import Path
import re

if __name__ == "__main__":
    spec = (Path(__file__).parent / "spec.txt").read_text("utf8").splitlines()
    tests = []
    pat = re.compile(r"^[`]{3,}\s*example")
    while spec:
        line = spec.pop(0)
        if pat.match(line):
            line = spec.pop(0)
            md, html = [], []
            in_md = True
            while not line.startswith("`" * 10):
                if line == ".":
                    in_md = False
                    line = spec.pop(0)
                    continue
                if in_md:
                    md.append(line)
                else:
                    html.append(line)
                line = spec.pop(0)
            assert not in_md
            tests.append((md, html))

    for i, (md, html) in enumerate(tests, 1):
        (Path(__file__).parent / "fixtures").mkdir(exist_ok=True)
        filename = f"spec_{i:03}.md"
        if i in (491, 620, 621):
            # these examples clash with the GFM extensions
            continue
        md_text = '\n'.join(md)
        html_text = '\n'.join(html)
        text = f"Example {i}\n......\n\n{md_text}\n\n......\n\n{html_text}\n"
        (Path(__file__).parent / "fixtures" / filename).write_text(text, "utf8")
