/**
 * [P001679] Max Number of K-Sum Pairs
 *
 * <p>给你一个整数数组 <code>nums</code> 和一个整数 <code>k</code> 。</p>

<p>每一步操作中，你需要从数组中选出和为 <code>k</code> 的两个整数，并将它们移出数组。</p>

<p>返回你可以对数组执行的最大操作数。</p>

<p> </p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,2,3,4], k = 5
<strong>输出：</strong>2
<strong>解释：</strong>开始时 nums = [1,2,3,4]：
- 移出 1 和 4 ，之后 nums = [2,3]
- 移出 2 和 3 ，之后 nums = []
不再有和为 5 的数对，因此最多执行 2 次操作。</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>nums = [3,1,3,4,3], k = 6
<strong>输出：</strong>1
<strong>解释：</strong>开始时 nums = [3,1,3,4,3]：
- 移出前两个 3 ，之后nums = [1,4,3]
不再有和为 6 的数对，因此最多执行 1 次操作。</pre>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 <= nums.length <= 10<sup>5</sup></code></li>
	<li><code>1 <= nums[i] <= 10<sup>9</sup></code></li>
	<li><code>1 <= k <= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/max-number-of-k-sum-pairs/
// discuss: https://leetcode.com/problems/max-number-of-k-sum-pairs/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001679() {
    }
}