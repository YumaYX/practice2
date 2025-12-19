use std::process::Command;

fn run_command(cmd: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stdout).to_string())     
    }
}

/// 出力加工関数
/// 今回は行単位で Rust ファイルを抽出する例
fn process_output(output: &str) -> Vec<String> {
    output
        .lines()
        .filter(|line| line.contains(".rs")) // Rust ファイルだけ抽出
        .map(|line| line.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_command_success() {
        let result = run_command("echo", &["hello"]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "hello"); // 改行を除去して比較
    }

    #[test]
    fn test_run_command_failure() {
        // 存在しないコマンドでエラーを発生させる
        let result = run_command("nonexistent_command", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_command_ls() {
        // ls が存在することを前提にしたテスト
        let result = run_command("ls", &["-1"]);
        assert!(result.is_ok());
        // 出力の先頭をチェック（空でないことを確認）
        let out = result.unwrap();
        assert!(!out.is_empty());
    }
}


