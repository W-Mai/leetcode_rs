/**
 * [P002563] Count the Number of Fair Pairs
 *
 * <p>给你一个下标从 <strong>0</strong> 开始、长度为 <code>n</code> 的整数数组&nbsp;<code>nums</code>&nbsp;，和两个整数&nbsp;<code>lower</code> 和&nbsp;<code>upper</code> ，返回 <strong>公平数对的数目</strong> 。</p>

<p>如果&nbsp;<code>(i, j)</code>&nbsp;数对满足以下情况，则认为它是一个 <strong>公平数对</strong>&nbsp;：</p>

<ul>
	<li><code>0 &lt;= i &lt; j &lt; n</code>，且</li>
	<li><code>lower &lt;= nums[i] + nums[j] &lt;= upper</code></li>
</ul>

<p>&nbsp;</p>

<p><b>示例 1：</b></p>

<pre>
<b>输入：</b>nums = [0,1,7,4,4,5], lower = 3, upper = 6
<b>输出：</b>6
<b>解释：</b>共计 6 个公平数对：(0,3)、(0,4)、(0,5)、(1,3)、(1,4) 和 (1,5) 。
</pre>

<p><b>示例 2：</b></p>

<pre>
<b>输入：</b>nums = [1,7,9,2,5], lower = 11, upper = 11
<b>输出：</b>1
<b>解释：</b>只有单个公平数对：(2,3) 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>nums.length == n</code></li>
	<li><code>-10<sup>9</sup>&nbsp;&lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>-10<sup>9</sup>&nbsp;&lt;= lower &lt;= upper &lt;= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/count-the-number-of-fair-pairs/
// discuss: https://leetcode.com/problems/count-the-number-of-fair-pairs/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002563() {
    }
}