次世代言語 Rust
===============

Programming Rust (2016)
http://shop.oreilly.com/product/0636920040385.do?sortby=bestSellers

コンパイル時間短縮，並列コード生成などを実現したRust 1.2
http://www.infoq.com/jp/news/2015/08/rust-12-released

Pythonプログラマの為のRust入門
http://qiita.com/t2y/items/434854fab16159a7c0f7

Rustを学びシステムレベル言語を理解すること
http://wazanova.jp/items/1580


Webのドキュメントはかなりしっかりしている
http://doc.rust-lang.org/book/

static linkingの仕方
https://doc.rust-lang.org/book/advanced-linking.html

### インストール

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

defaultではstableの1


### Vim plugin

.vimrcに、Vundleの設定を書く。

    Bundle 'rust-lang/rust.vim'
    :BundleInstall

して、

    au BufRead,BufNewFile,BufReadPre *.rs   set filetype=rust

をvimrcに書く。

### VS Code


### プロジェクト生成

Cargoというツールが標準で付属している

    cargo new hello_rust --bin


### REPL

最新のrust 1.2.0では動かないくさい

https://github.com/murarth/rusti


### Web Assembly

http://qiita.com/_likr/items/daf46d6f66bc31cc4810


