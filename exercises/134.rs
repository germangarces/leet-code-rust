struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (total_tank, start, _) = gas
            .iter()
            .zip(cost.iter()).enumerate()
            .fold((0, 0, 0), |(total, start, current), (i, (&g, &c))| {
                let diff = g - c;
                let total = total + diff;
                let current = current + diff;
                if current < 0 {
                    (total, i + 1, 0)
                } else {
                    (total, start, current)
                }
            },
        );

        if total_tank < 0 { -1 } else { start as i32 }
    }
}

fn main() {
    let gas = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];
    let result = Solution::can_complete_circuit(gas, cost);
    assert_eq!(result, 3);

    let gas = vec![2, 3, 4];
    let cost = vec![3, 4, 3];
    let result = Solution::can_complete_circuit(gas, cost);
    assert_eq!(result, -1);
}

