# HL (WIP)
Syntax highlighting written in Rust. This project is intended to be used to generate html syntax highlighting from the given file.

## Usage
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
    -g, --generate, generate    Generate lexer.
    help                        Print this message or the help of the given subcommand(s)
```

### Example Command
```bash
hl example/rust.rs -l rust
```

### Example Result
```html
<table class="highlight-table">
<tbody>
<tr><td class="hl-num" data-line="1"></td><td>// The comment section</td></tr>
<tr><td class="hl-num" data-line="2"></td><td><span class="hl-k">fn</span> <span class="hl-en">main</span>() {</td></tr>
<tr><td class="hl-num" data-line="3"></td><td>    <span class="hl-k">let</span> matches = App::<span class="hl-en">new</span>(<span class="hl-c">"hl"</span>)</td></tr>
<tr><td class="hl-num" data-line="4"></td><td>        .<span class="hl-en">version</span>(<span class="hl-c">"0.1.0"</span>);</td></tr>
<tr><td class="hl-num" data-line="5"></td><td>    <span class="hl-k">let</span> ada = <span class="hl-c">5</span>;</td></tr>
<tr><td class="hl-num" data-line="6"></td><td>    <span class="hl-k">if</span> <span class="hl-c">true</span> {</td></tr>
<tr><td class="hl-num" data-line="7"></td><td>    }</td></tr>
<tr><td class="hl-num" data-line="8"></td><td>}</td></tr>
<tr><td class="hl-num" data-line="9"></td><td></td></tr>
<tr><td class="hl-num" data-line="10"></td><td><span class="hl-k">fn</span> <span class="hl-en">process</span>(a: &<span class="hl-k">str</span>, b: <span class="hl-k">char</span>) {</td></tr>
<tr><td class="hl-num" data-line="11"></td><td>    <span class="hl-en">println!</span>(a, b);</td></tr>
<tr><td class="hl-num" data-line="12"></td><td>}</td></tr>
<tr><td class="hl-num" data-line="13"></td><td></td></tr>
</tbody>
</table>
```