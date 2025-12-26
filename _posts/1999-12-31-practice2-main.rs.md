---
layout: post
category: practice2
title: "main.rs"
---

<small>この文章はAIで生成しています。誤りが含まれることがあります。</small>

このRustプログラムは、指定されたRustコードファイルを処理し、それらのコードを説明するMarkdownファイルを生成します。以下がその詳細な説明です。

1. **モジュールのインポート**:
   - `ollama` モジュールをインポートし、`request_ollama` 関数を使用します。
   - `glob` クレートを使用してファイルパターンマッチングを行います。
   - 標準ライブラリ (`std`) の `fs`, `io`, `path` モジュールをインポートします。

2. **非同期エントリポイント**:
   - `main` 関数は非同期関数であり、`run` 関数を呼び出します。エラーが発生した場合はエラーメッセージを出力します。

3. **メイン処理**:
   - `_posts` ディレクトリを作成します。
   - `list_sample_rs_ファイル` 関数で指定されたパターンに一致するRustファイルのリストを取得します。
   - 各Rustファイルに対して `process_rust_file` 関数を呼び出します。

4. **Rustファイルの処理**:
   - `process_rust_file` 関数は、Rustファイルのカテゴリ、ファイル名、説明Markdownファイルのパスを抽出します。
   - Jekyllのフロントマターを生成し、既存の説明Markdownファイルが存在する場合は読み込みます。
   - Rustソースコードを読み込み、説明Markdownファイルが存在しない場合はOllamaに問い合わせて説明を生成します。
   - Rustコードブロックを追加し、Jekyllポストを出力します。

5. **ユーティリティ関数**:
   - `extract_カテゴリ` 関数はファイルのカテゴリを抽出します。
   - `extract_ファイル名` 関数はファイル名を抽出します。
   - `build_ポストパス` 関数はポストのパスを構築します。
   - `jekyll_ポストテンプレート` 関数はJekyllのフロントマターを生成します。
   - `prompt_説明コード` 関数は説明コードのプロンプトを生成します。
   - `read_ファイル` 関数はファイルの内容を読み込みます。
   - `write_ファイル` 関数はファイルに内容を書き込みます。
   - `list_サンプル_ファイル` 関数はサンプルRustファイルのリストを取得します。

このプログラムは、Rustコードを処理し、それらのコードを説明するMarkdownファイルを生成するためのツールです。各Rustファイルは説明Markdownファイルが存在しない場合、Ollamaに問い合わせて説明を生成します。生成された説明とRustコードブロックを含むJekyllポストを出力します。

```rust
mod ollama;
use crate::ollama::request_ollama;

use glob::glob;
use std::{
    fs,
    io,
    path::{Path, PathBuf},
};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
    }
}

async fn run() -> io::Result<()> {
    let posts_dir = Path::new("_posts");
    fs::create_dir_all(posts_dir)?;

    for rs_path in list_sample_rs_files() {
        process_rust_file(&rs_path).await?;
    }

    Ok(())
}

async fn process_rust_file(rs_path: &Path) -> io::Result<()> {
    let category = extract_category(rs_path)?;
    let file_name = extract_filename(rs_path)?;

    let description_md_path = rs_path.with_extension("md");
    let mut post_content = String::new();

    // Jekyll front matter
    post_content.push_str(&jekyll_posts_template(&category, &file_name));

    // 既存の説明 Markdown があれば読み込む
    if description_md_path.exists() {
        if let Ok(existing_md) = read_file(&description_md_path) {
            post_content.push_str(&existing_md);
        }
    }

    // Rust ソース読み込み
    let rust_source = read_file(rs_path)?;

    // 説明 Markdown が存在しない場合のみ Ollama に問い合わせ
    if !description_md_path.exists() {
        let prompt = format!(
            "{}\n---\n{}",
            prompt_to_describe_codes(),
            rust_source
        );

        println!("{}", &prompt);
        let description = request_ollama(&prompt, None).await;
        write_file(&description_md_path, &description)?;
        post_content.push_str(&description);
    }

    // Rust コードブロック追加
    post_content.push_str("\n\n```rust\n");
    post_content.push_str(&rust_source);
    post_content.push_str("```\n");

    // Jekyll post 出力
    let output_path = build_post_path(&category, &file_name);
    write_file(&output_path, &post_content)?;

    println!("Generated post: {}", output_path.display());
    Ok(())
}

fn extract_category(path: &Path) -> io::Result<String> {
    path.parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Category not found"))
}

fn extract_filename(path: &Path) -> io::Result<String> {
    path.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Filename not found"))
}

fn build_post_path(category: &str, file_name: &str) -> PathBuf {
    PathBuf::from(format!(
        "_posts/1999-12-31-{}-{}.md",
        category, file_name
    ))
}

fn jekyll_posts_template(category: &str, title: &str) -> String {
    format!(
        r#"---
layout: post
category: {}
title: "{}"
---

<small>この文章はAIで生成しています。誤りが含まれることがあります。</small>

"#,
        category, title
    )
}

fn prompt_to_describe_codes() -> String {
    r#"
- Write about the following Rust codes in Japanese using markdown format.
- In detail.
- Just the answer.
- dont use code block
- Start your response with 'このRustプログラムは'.
"#
    .to_string()
}

fn read_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

fn write_file(path: &Path, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

fn list_sample_rs_files() -> Vec<PathBuf> {
    glob("sample/*/*.rs")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .collect()
}

```
