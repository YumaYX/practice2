---
layout: post
category: sample
title: "parapoly.rs"
---

<small>この文章はAIで生成しています。誤りが含まれることがあります。</small>

このRustプログラムは、文字列の長さを計算し、出力する関数`print_len`を定義しています。この関数はジェネリック型`T`を受け取り、`AsRef<str>`トレイトを実装している型に対してのみ使用できます。これにより、文字列リテラルや`String`型の両方に対して`print_len`関数を呼び出すことができます。

`main`関数では、`print_len`関数を使用して文字列"hello"と`String::from("world")`の長さを計算し、出力しています。このプログラムは、Rustのジェネリック型とトレイトの利用方法を示し、文字列操作の基本的な操作を実装しています。

```rust

fn print_len<T: AsRef<str>>(s: T) {
    println!("{}", s.as_ref().len());
}

fn main () {
    print_len("hello");
    print_len(String::from("world"));
}
```
