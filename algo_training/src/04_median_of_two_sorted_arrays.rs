struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged: Vec<i32> = vec![];
        merged.extend(&nums1);
        merged.extend(&nums2);
        merged.sort();
        let len = merged.len();
        let mid = len / 2;
        if len % 2 == 1 {
            return merged[mid] as f64
        }
        (merged[mid] as f64 + merged[mid - 1] as f64) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(result, 2.0);

        let result = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(result, 2.5);

        let result = Solution::find_median_sorted_arrays(vec![], vec![1]);
        assert_eq!(result, 1.0);

        let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 7]);
        assert_eq!(result, 2.5);
    }
}