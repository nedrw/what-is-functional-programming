// 演示：使用 rayon 将迭代器链一键并行化
// 不可变数据流 + 无共享可变状态 = 无锁并发
//
// 核心价值：
// - 从串行到并行，只需改 .iter() → .par_iter()
// - 不可变性 + 纯函数 = 自动保证线程安全，无需锁/Mutex
// - 业务代码无需关心并行细节，库负责执行策略
//
// 性能说明：
// - 本示例的计算任务较轻（x*x），并行开销可能抵消收益
// - 重点不在于"更快"，而在于"安全地改变执行策略"

use rayon::prelude::*;

/// 纯函数：计算平方和（可测试）
fn compute_sum(data: &[u64]) -> u64 {
    data.iter().map(|x| x * x).sum()
}

/// 并行版本：同样的逻辑，并行执行
fn compute_sum_parallel(data: &[u64]) -> u64 {
    data.par_iter().map(|x| x * x).sum()
}

fn main() {
    let data: Vec<u64> = (0..1_000_000).collect();

    // 验证串行和并行结果一致
    let serial = compute_sum(&data);
    let parallel = compute_sum_parallel(&data);
    assert_eq!(serial, parallel);

    println!("✅ 结果一致：serial={}, parallel={}", serial, parallel);
    println!("✅ 并行计算正确，无需锁或 Mutex");

    // 性能对比（仅供参考，单次测量不精确）
    let start = std::time::Instant::now();
    let _ = compute_sum(&data);
    let t_serial = start.elapsed();

    let start = std::time::Instant::now();
    let _ = compute_sum_parallel(&data);
    let t_parallel = start.elapsed();

    println!("\n性能对比（仅供参考）：");
    println!("  串行耗时: {:?}", t_serial);
    println!("  并行耗时: {:?}", t_parallel);
    if t_parallel < t_serial {
        println!(
            "  加速比: {:.2}x",
            t_serial.as_secs_f64() / t_parallel.as_secs_f64()
        );
    } else {
        println!("  注意: 此计算任务过轻，并行开销大于收益");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_and_parallel_produce_same_result() {
        let data: Vec<u64> = vec![1, 2, 3, 4, 5];
        assert_eq!(compute_sum(&data), compute_sum_parallel(&data));
    }

    #[test]
    fn test_compute_sum() {
        let data: Vec<u64> = vec![1, 2, 3];
        // 1*1 + 2*2 + 3*3 = 1 + 4 + 9 = 14
        assert_eq!(compute_sum(&data), 14);
    }

    #[test]
    fn test_empty_data() {
        let data: Vec<u64> = vec![];
        assert_eq!(compute_sum(&data), 0);
        assert_eq!(compute_sum_parallel(&data), 0);
    }
}
