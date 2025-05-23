# Thus Saith My Waifu!

[![Crates.io](https://img.shields.io/crates/v/thus-saith.svg)](https://crates.io/crates/thus-saith)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Thus saith my waifu, I mean it.

## Installation

```
cargo install thus-saith
```

## Usage

```
Thus saith my waifu!

Usage: thus-saith [OPTIONS]

Options:
      --mean <NUMBER>     Average time per character (unit: ms)
      --stddev <NUMBER>  Standard deviation of time per character (unit: ms)
      --config <FILE>     Load the specified configuration file
  -h, --help              Print help
  -V, --version           Print version
```

## Configuration

You can create configuration files at these locations:

1. `thus-saith.toml` in current working directory.
2. `thus-saith/config.toml` in [user config directory](https://docs.rs/dirs/6.0.0/dirs/fn.config_dir.html).

Refer to the [default configuration file](config/default.toml) for more details.

## Examples

```shell
❯ thus-saith
啊嘞啊嘞 QAQ？多洗忒 …… 欧尼酱 ww？
呐、桥豆麻袋 …… 已经「厌烦」吾辈了嘛？
哼唧 …… 真是「冷·酷·の·人」呢 QuQ —— ☆(๑°⌓°๑)
嘛 …… 即便是这样的瓦塔西，一定也是有「存·在·の·意·义」的吧、内 ~ ★
快来「肯定」啊？不然呀 …… 咱可就要「黑化」了哦 ♪ 呐？
```

```shell
❯ thus-saith --mean 20 --stddev 40
呐、二次元の民那 …… 都·是·最·最·善·良·の·存·在·呐 ☆
多洗忒 …… 要「嘲笑」这样的孩子呢？吾辈不明白啊 ——
嘛 …… 说到底，你们都只是污秽の「来自三次元的大人」吧？
大人什么的、最·讨·厌·了 ★ ♪
```

```shell
❯ cat << HERE > foo.bar
[[quote]]
weight = 1
content = """
诶多 …… 看起来阁下对于「二·次·元」の理解、似·乎·满·是·谬·误·哦 ☆ ~
嘛，连最为基本の「礼♪义♪廉♪耻♪」都早已失去了啊 …… ♪（笑）
呐，我说啊 —— 这样の kimino、也有自称「二 ♡ 次 ♡ 元」の资格吗 ★ ？
fufufu —— 说到底、阁下已经「二·次·元·失·格」了吧？呐 ~ ♪
"""
HERE

❯ thus-saith --config foo.bar
诶多 …… 看起来阁下对于「二·次·元」の理解、似·乎·满·是·谬·误·哦 ☆ ~
嘛，连最为基本の「礼♪义♪廉♪耻♪」都早已失去了啊 …… ♪（笑）
呐，我说啊 —— 这样の kimino、也有自称「二 ♡ 次 ♡ 元」の资格吗 ★ ？
fufufu —— 说到底、阁下已经「二·次·元·失·格」了吧？呐 ~ ♪
```
