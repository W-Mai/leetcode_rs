/**
 * [P000891] Sum of Subsequence Widths
 *
 * <p>一个序列的 <strong>宽度</strong> 定义为该序列中最大元素和最小元素的差值。</p>

<p>给你一个整数数组 <code>nums</code> ，返回 <code>nums</code> 的所有非空 <strong>子序列</strong> 的 <strong>宽度之和</strong> 。由于答案可能非常大，请返回对 <code>10<sup>9</sup> + 7</code> <strong>取余</strong> 后的结果。</p>

<p><strong>子序列</strong> 定义为从一个数组里删除一些（或者不删除）元素，但不改变剩下元素的顺序得到的数组。例如，<code>[3,6,2,7]</code> 就是数组 <code>[0,3,1,6,2,2,7]</code> 的一个子序列。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>nums = [2,1,3]
<strong>输出：</strong>6
<strong>解释：</strong>子序列为 [1], [2], [3], [2,1], [2,3], [1,3], [2,1,3] 。
相应的宽度是 0, 0, 0, 1, 1, 2, 2 。
宽度之和是 6 。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>nums = [2]
<strong>输出：</strong>0
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/sum-of-subsequence-widths/
// discuss: https://leetcode.com/problems/sum-of-subsequence-widths/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn sum_subseq_widths(nums: Vec<i32>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000891() {
    }
}