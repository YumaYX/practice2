---
layout: post
category: fileio
title: "fileio.rs"
---

<small>この文章はAIで生成しています。誤りが含まれることがあります。</small>

このRustプログラムは、ファイルの読み書き操作を実行するシンプルな例です。`std::fs`モジュールを使用して、ファイルシステムの操作を行います。`read_file`関数は指定されたパスのファイルを読み込み、`write_file`関数は指定されたパスに内容を書き込みます。`main`関数では、まずファイルに「Hello, world!」という内容を書き込み、成功または失敗の結果を出力します。次に、書き込んだファイルを読み込み、その内容を出力します。最後に、ファイルを削除し、その成功または失敗の結果を出力します。

このプログラムでは、Rustのエラーハンドリングの方法が重要です。`match`式を使用して、`Result`型の戻り値を処理し、成功または失敗の結果を適切に処理します。また、`if let`式を使用して、ファイルの削除操作の結果を処理します。このプログラムは、Rustの基本的なファイル操作とエラーハンドリングの方法を示しています。

```rust
use std::{
    fs,
    io,
    path::Path,
};

fn read_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

fn write_file(path: &Path, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

fn main() {
    let path = Path::new("a");

    // WRITE
    let wf = write_file(path, "Hello, world!");
    match wf {
        Ok(()) => println!("write - success"),
        Err(e) => eprintln!("write - fail: {}", e),
    }

    // READ
    let rf = read_file(path);
    match rf {
        Ok(content) => println!("read - success {}",content),
        Err(err) => eprintln!("read - fail {}",err),
    }

    if let Err(e) = fs::remove_file(path) {
        eprintln!("file del - fail: {}", e);
    } else {
        println!("file del - success");
    }
}


```
