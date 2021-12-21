# HL (WIP)
Syntax highlighting.

## Usage
```bash
hl example/rust.rs
```
### Example result
```html
<table class="highlight-table">
  <tbody>
  <tr><td class="hl-num" data-line="1"></td><td></td></tr>
  <tr><td class="hl-num" data-line="2"></td><td><span class="hl-k">fn</span> <span class="hl-id">main</span>() {</td></tr>
  <tr><td class="hl-num" data-line="3"></td><td>    <span class="hl-k">let</span> <span class="hl-id">matches</span> <span class="hl-op">=</span> <span class="hl-id">App</span>::<span class="hl-id">new</span>(<span class="hl-s">"hl"</span>)</td></tr>
  <tr><td class="hl-num" data-line="4"></td><td>        .<span class="hl-id">version</span>(<span class="hl-s">"0.1.0"</span>);</td></tr>
  <tr><td class="hl-num" data-line="5"></td><td>    <span class="hl-k">let</span> <span class="hl-id">ada</span> <span class="hl-op">=</span> <span class="hl-val">5</span>;</td></tr>
  <tr><td class="hl-num" data-line="6"></td><td>    <span class="hl-k">if</span> <span class="hl-c">true</span> {</td></tr>
  <tr><td class="hl-num" data-line="7"></td><td>    }</td></tr>
  <tr><td class="hl-num" data-line="8"></td><td>}</td></tr>
  <tr><td class="hl-num" data-line="9"></td><td></td></tr>
  </tbody>
</table>
```