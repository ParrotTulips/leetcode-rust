use leetcode_rust::Solution;
#[cfg(test)]
mod tests {
    use super::*;

    // 工具：把返回的两个下标排序，避免 [i,j] 与 [j,i] 的顺序差异
    fn sorted_pair(mut v: Vec<i32>) -> Vec<i32> {
        v.sort_unstable();
        v
    }

    // 官方样例
    #[test]
    fn example_1() {
        let res = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(sorted_pair(res), vec![0, 1]);
    }

    #[test]
    fn example_2() {
        let res = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(sorted_pair(res), vec![1, 2]);
    }

    #[test]
    fn example_3() {
        let res = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(sorted_pair(res), vec![0, 1]);
    }

    // 进阶覆盖：负数、零、重复元素等
    #[test]
    fn with_negatives() {
        let res = Solution::two_sum(vec![-1, -2, -3, -4, -5], -8);
        assert_eq!(sorted_pair(res), vec![2, 4]); // -3 + -5
    }

    #[test]
    fn includes_zeroes() {
        let res = Solution::two_sum(vec![0, 4, 3, 0], 0);
        assert_eq!(sorted_pair(res), vec![0, 3]); // 0 + 0
    }

    #[test]
    fn duplicates_values() {
        let res = Solution::two_sum(vec![1, 5, 1, 5], 10);
        assert_eq!(sorted_pair(res), vec![1, 3]); // 5 + 5
    }

    #[test]
    fn mixed_signs() {
        let res = Solution::two_sum(vec![1, 4, 10, -3], 14);
        assert_eq!(sorted_pair(res), vec![1, 2]); // 4 + 10
    }
}
