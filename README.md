# 🌴 HL (WIP)
Syntax highlighting written in Rust. The project is designed to generate html syntax highlighting for the given file.

This software is inspired by github syntax highlighter.

## 🚀 Usage
```bash
USAGE:
    hl [OPTIONS] [FILE_PATH] [SUBCOMMAND]

ARGS:
    <FILE_PATH>    File path to parse.

OPTIONS:
    -h, --help          Print help information
    -l <LANG>...        Language.
    -V, --version       Print version information

SUBCOMMANDS:
    -g, --generate, generate    Generate lexer, this is for development only.
    help                        Print this message or the help of the given subcommand(s)

```

### 💡 Example Command
```bash
cargo run --package hl --example html examples/html.rs
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
1. [x] ActionScript ( ✅ Done )
2. [x] Bash ( ✅ Done )
3. [x] C ( ✅ Done )
4. [ ] Clojure ( 🚧 In Progress )
5. [x] CSS ( ✅ Done )
6. [x] CUDA ( ✅ Done )
7. [x] Dart ( ✅ Done )
8. [x] CPP ( ✅ Done )
9. [x] C# ( ✅ Done )
10. [x] Go ( ✅ Done )
11. [ ] HTML ( 🚧 In Progress )
12. [x] Ruby ( ✅ Done )
13. [ ] Rust ( 🚧 In Progress )
14. [x] Java ( ✅ Done )
15. [x] Javascript ( ✅ Done )
16. [x] JSON ( ✅ Done )
17. [x] Kotlin ( ✅ Done )
18. [x] Lua ( ✅ Done )
19. [x] Markdown ( ✅ Done )
20. [x] Nim ( ✅ Done )
21. [x] PHP ( ✅ Done )
22. [ ] Python ( 🚧 In Progress )
23. [x] Typescript ( ✅ Done )
22. [ ] Vu ( 🚧 In Progress )
24. [x] Yaml ( ✅ Done )
25. [x] Zig ( ✅ Done )
