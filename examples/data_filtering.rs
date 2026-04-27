// 演示：使用迭代器组合子进行数据筛选与变换
// 无 mut、无 for 循环，全程声明式

/// 一个虚构的订单记录
#[derive(Debug)]
struct Order {
    id: u32,
    amount: f64,
    category: String,
}

fn main() {
    let orders = vec![
        Order {
            id: 1,
            amount: 25.0,
            category: "food".into(),
        },
        Order {
            id: 2,
            amount: 15.0,
            category: "food".into(),
        },
        Order {
            id: 3,
            amount: 50.0,
            category: "books".into(),
        },
        Order {
            id: 4,
            amount: 5.0,
            category: "food".into(),
        },
        Order {
            id: 5,
            amount: 30.0,
            category: "books".into(),
        },
    ];

    let total_food_sales: f64 = orders
        .iter()
        .filter(|o| o.category == "food")
        .map(|o| o.amount)
        .sum();

    println!("食物类订单原始数据：");
    orders
        .iter()
        .filter(|o| o.category == "food")
        .for_each(|o| println!("  Order #{}: ${:.2}", o.id, o.amount));

    println!("\n食物类总销售额: ${:.2}", total_food_sales);

    // fold: 手动聚合，将数据"折叠"成单一值
    let category_counts = orders
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, order| {
            *acc.entry(&order.category).or_insert(0) += 1;
            acc
        });

    println!("\n各类别订单数：{:?}", category_counts);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_and_sum() {
        let orders = vec![
            Order {
                id: 1,
                amount: 10.0,
                category: "food".into(),
            },
            Order {
                id: 2,
                amount: 20.0,
                category: "books".into(),
            },
            Order {
                id: 3,
                amount: 30.0,
                category: "food".into(),
            },
        ];

        let total: f64 = orders
            .iter()
            .filter(|o| o.category == "food")
            .map(|o| o.amount)
            .sum();

        assert_eq!(total, 40.0);
    }

    #[test]
    fn test_fold_category_counts() {
        let orders = vec![
            Order {
                id: 1,
                amount: 10.0,
                category: "food".into(),
            },
            Order {
                id: 2,
                amount: 20.0,
                category: "books".into(),
            },
            Order {
                id: 3,
                amount: 30.0,
                category: "food".into(),
            },
        ];

        let counts = orders
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, order| {
                *acc.entry(order.category.clone()).or_insert(0) += 1;
                acc
            });

        assert_eq!(*counts.get("food").unwrap(), 2);
        assert_eq!(*counts.get("books").unwrap(), 1);
    }
}
