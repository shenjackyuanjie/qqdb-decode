# qqdb 解码

简称:

[qq_msg_decode](https://github.com/saucer-man/qq_msg_decode) 但是 Rewrite in Rust

感谢 icalingua 现任维护者 和他的 [oicq-icalingua-plus-plus](https://github.com/Icalingua-plus-plus/oicq-icalingua-plus-plus/blob/a8fa0e6e2448a626491cf365e1beb23f2e82a509/lib/message/parser.js#L187)

## 用法

还没写好

反正你

```powershell
cargo run --release xxx.db
```

应该就行
大不了就是

```powershell
cargo run --release -- xxx.db -o xxx.db/txt?/csv?
```

也有可能我最后写成把数据写入到另一个数据库里面
比如写入到某个 PostgreSQL 里 ( pg 教徒是这样的 )
