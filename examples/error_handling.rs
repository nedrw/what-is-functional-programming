// 演示：用 Option / Result 的组合子构建错误处理管道
// 避免深层 match，使用 ? 提早退出时的可读性

/// 将字符串解析为数字，可能失败
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("'{}'不是数字: {}", s, e))
}

/// 只接受正数
fn require_positive(n: i32) -> Result<i32, String> {
    if n > 0 {
        Ok(n)
    } else {
        Err(format!("{}不是正数", n))
    }
}

fn main() {
    let inputs = vec!["10", "-3", "abc", "42"];

    println!("处理输入串：{:?}", inputs);

    // 方法 1: 使用 and_then 链（显式函数组合）
    for input in &inputs {
        let result = parse_number(input).and_then(require_positive);
        match result {
            Ok(n) => println!("  ✅ 有效正数: {}", n),
            Err(e) => println!("  ❌ 错误: {}", e),
        }
    }

    // 方法 2: 使用 ? 运算符（更符合 Rust 惯用风格，但同样纯函数式思维）
    println!("\n使用 ? 运算符处理同一批数据：");
    for input in &inputs {
        match process_single(input) {
            Ok(_) => (),
            Err(e) => println!("  ❌ 错误: {}", e),
        }
    }
}

/// 针对单个输入的完整处理，展示了 ? 的"提前返回"功能，
/// 但对外部来说这个函数保持了纯函数的形式（无副作用，结果只由参数决定）。
fn process_single(input: &str) -> Result<(), String> {
    let n = parse_number(input)?;
    let n = require_positive(n)?;
    println!("  ✅ 有效正数: {}", n);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_valid() {
        assert_eq!(parse_number("42"), Ok(42));
        assert_eq!(parse_number("-10"), Ok(-10));
        assert_eq!(parse_number("0"), Ok(0));
    }

    #[test]
    fn test_parse_number_invalid() {
        assert!(parse_number("abc").is_err());
        assert!(parse_number("12.34").is_err());
        assert!(parse_number("").is_err());
    }

    #[test]
    fn test_require_positive() {
        assert_eq!(require_positive(10), Ok(10));
        assert_eq!(require_positive(1), Ok(1));
        assert!(require_positive(0).is_err());
        assert!(require_positive(-5).is_err());
    }

    #[test]
    fn test_and_then_chain() {
        // 有效正数
        assert_eq!(parse_number("42").and_then(require_positive), Ok(42));

        // 有效数字但非正数
        assert!(parse_number("-5").and_then(require_positive).is_err());

        // 无效数字
        assert!(parse_number("abc").and_then(require_positive).is_err());
    }
}
