/**
 * [P002607] Make K-Subarray Sums Equal
 *
 * <p>给你一个下标从 <strong>0</strong> 开始的整数数组 <code>arr</code> 和一个整数 <code>k</code> 。数组 <code>arr</code> 是一个循环数组。换句话说，数组中的最后一个元素的下一个元素是数组中的第一个元素，数组中第一个元素的前一个元素是数组中的最后一个元素。</p>

<p>你可以执行下述运算任意次：</p>

<ul>
	<li>选中 <code>arr</code> 中任意一个元素，并使其值加上 <code>1</code> 或减去 <code>1</code> 。</li>
</ul>

<p>执行运算使每个长度为 <code>k</code> 的 <strong>子数组</strong> 的元素总和都相等，返回所需要的最少运算次数。</p>

<p><strong>子数组</strong> 是数组的一个连续部分。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>arr = [1,4,1,3], k = 2
<strong>输出：</strong>1
<strong>解释：</strong>在下标为 1 的元素那里执行一次运算，使其等于 3 。
执行运算后，数组变为 [1,3,1,3] 。
- 0 处起始的子数组为 [1, 3] ，元素总和为 4 
- 1 处起始的子数组为 [3, 1] ，元素总和为 4 
- 2 处起始的子数组为 [1, 3] ，元素总和为 4 
- 3 处起始的子数组为 [3, 1] ，元素总和为 4 
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>arr = [2,5,5,7], k = 3
<strong>输出：</strong>5
<strong>解释：</strong>在下标为 0 的元素那里执行三次运算，使其等于 5 。在下标为 3 的元素那里执行两次运算，使其等于 5 。
执行运算后，数组变为 [5,5,5,5] 。
- 0 处起始的子数组为 [5, 5, 5] ，元素总和为 15
- 1 处起始的子数组为 [5, 5, 5] ，元素总和为 15
- 2 处起始的子数组为 [5, 5, 5] ，元素总和为 15
- 3 处起始的子数组为 [5, 5, 5] ，元素总和为 15
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= arr.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= arr[i] &lt;= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/make-k-subarray-sums-equal/
// discuss: https://leetcode.com/problems/make-k-subarray-sums-equal/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002607() {
    }
}