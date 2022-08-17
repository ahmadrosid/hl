# 🌴 HL (WIP)
Syntax highlighting written in Rust inspired by github syntax highlighter. The project is designed to generate html syntax highlighting. Originally I used at [heline.dev](https://heline.dev).

### 💡 Example
```bash
cargo run --package hl --example html examples/html.rs -l rust
```

### 💡 Example Result
```html
<table class="highlight-table">
<tbody>
<tr><td class="hl-num" data-line="1"></td><td><span class="hl-cmt">// The comment section</span></td></tr>
<tr><td class="hl-num" data-line="2"></td><td><span class="hl-k">fn</span> <span class="hl-en">main</span>() {</td></tr>
<tr><td class="hl-num" data-line="3"></td><td>    <span class="hl-k">let</span> matches = App::<span class="hl-en">new</span>(<span class="hl-c">"hl"</span>)</td></tr>
<tr><td class="hl-num" data-line="4"></td><td>        .<span class="hl-en">version</span>(<span class="hl-c">"0.1.0"</span>);</td></tr>
<tr><td class="hl-num" data-line="5"></td><td>    <span class="hl-k">let</span> ada = <span class="hl-c">5</span>;</td></tr>
<tr><td class="hl-num" data-line="6"></td><td>    <span class="hl-k">if</span> <span class="hl-c">true</span> {</td></tr>
<tr><td class="hl-num" data-line="7"></td><td>        <span class="hl-k">String</span>::<span class="hl-en">new</span>();</td></tr>
<tr><td class="hl-num" data-line="8"></td><td>        <span class="hl-k">let</span> a : <span class="hl-k">Vec</span><<span class="hl-k">char</span>> = <span class="hl-en">vec</span>!['0'];</td></tr>
<tr><td class="hl-num" data-line="9"></td><td>    }</td></tr>
<tr><td class="hl-num" data-line="10"></td><td>}</td></tr>
<tr><td class="hl-num" data-line="11"></td><td></td></tr>
<tr><td class="hl-num" data-line="12"></td><td><span class="hl-k">fn</span> <span class="hl-en">process</span>(a: &<span class="hl-k">str</span>, b: <span class="hl-k">char</span>) {</td></tr>
<tr><td class="hl-num" data-line="13"></td><td>    <span class="hl-en">println</span>!(a, b);</td></tr>
<tr><td class="hl-num" data-line="14"></td><td>}</td></tr>
<tr><td class="hl-num" data-line="15"></td><td></td></tr>
</tbody>
</table>
```

See the result in browser [here](https://play.tailwindcss.com/JevzqYGpuH).

## Lexers
- [x] ActionScript ( ✅ Done )
- [x] Ada ( ✅ Done )
- [x] Bash ( ✅ Done )
- [x] C ( ✅ Done )
- [ ] Clojure ( 🚧 In Progress )
- [x] CSS ( ✅ Done )
- [x] CUDA ( ✅ Done )
- [x] Dart ( ✅ Done )
- [x] CPP ( ✅ Done )
- [x] C# ( ✅ Done )
- [x] Go ( ✅ Done )
- [ ] HTML ( 🚧 In Progress )
- [x] Ruby ( ✅ Done )
- [ ] Rust ( 🚧 In Progress )
- [x] Java ( ✅ Done )
- [x] Javascript ( ✅ Done )
- [x] JSON ( ✅ Done )
- [x] Kotlin ( ✅ Done )
- [x] Lua ( ✅ Done )
- [x] Markdown ( ✅ Done )
- [x] Nim ( ✅ Done )
- [x] PHP ( ✅ Done )
- [ ] Python ( 🚧 In Progress )
- [x] Typescript ( ✅ Done )
- [ ] Vue ( 🚧 In Progress )
- [x] Yaml ( ✅ Done )
- [x] Zig ( ✅ Done )
- [x] Toml ( ✅ Done )
- [x] Lua ( ✅ Done )
- [x] Groovy ( ✅ Done )
- [x] Makefile ( ✅ Done )
- [x] Erlang ( ✅ Done )
- [x] Vlang ( ✅ Done )
- [x] Coffescript ( ✅ Done )
- [x] Protocol Buffer ( ✅ Done )