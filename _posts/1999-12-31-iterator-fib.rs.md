---
layout: post
category: iterator
title: "fib.rs"
---

<small>この文章はAIで生成しています。誤りが含まれることがあります。</small>

このRustプログラムは、フィボナッチ数列を生成するイテレータを実装し、その中から特定の位置の数値を取得するプログラムです。プログラムは、フィボナッチ数列を生成するための構造体 `Fib` を定義し、そのイテレータとして実装しています。`Fib` 構造体は、フィボナッチ数列の2つの前の数値 `a` とその次の数値 `b` を保持しています。`Fib` 構造体は `Iterator` トレイトを実装しており、`next` メソッドを実装することで、フィボナッチ数列の次の数値を生成します。`fib_iter` 関数は、`Fib` 構造体のインスタンスを生成し、フィボナッチ数列を生成するイテレータを返します。`main` 関数では、`fib_iter` 関数を呼び出し、50番目のフィボナッチ数を取得し、出力します。

このプログラムは、Rustの構造体とイテレータの機能を活用して、効率的にフィボナッチ数列を生成し、特定の位置の数値を取得しています。Rustの構造体は、データを保持し、関連する操作を実装するための方法を提供します。イテレータは、シーケンスの要素を順番に処理するための方法を提供します。このプログラムは、Rustの構造体とイテレータの機能を活用して、効率的にフィボナッチ数列を生成し、特定の位置の数値を取得しています。

```rust
struct Fib {
    a: u128,
    b: u128,
}

impl Iterator for Fib {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(v)
    }
}

fn fib_iter() -> Fib {
    Fib { a: 0, b: 1 }
}

fn main() {
    let x = fib_iter().nth(50).unwrap();
    println!("{}", x);
}


```
