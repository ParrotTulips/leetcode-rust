use leetcode_rust::Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let result = Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(result, 2.0);
    }
}
