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
hl src/main.rs -l rust
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
- [x] Bash ( ✅ Done )
- [x] C ( ✅ Done )
- [ ] Clojure ( 🚧 In Progress )
- [x] CSS ( ✅ Done )
- [x] CUDA ( ✅ Done )
- [x] CPP ( ✅ Done )
- [x] C# ( ✅ Done )
- [ ] Go ( 🚧 In Progress )
- [ ] HTML ( 🚧 In Progress )
- [ ] Rust ( 🚧 In Progress )
- [x] Java ( ✅ Done )
- [x] Javascript ( ✅ Done )
- [x] JSON ( ✅ Done )
- [x] Lua ( ✅ Done )
- [x] Markdown ( ✅ Done )
- [x] PHP ( ✅ Done )
- [ ] Python ( 🚧 In Progress )
- [x] Typescript ( ✅ Done )
- [x] Yaml ( ✅ Done )
