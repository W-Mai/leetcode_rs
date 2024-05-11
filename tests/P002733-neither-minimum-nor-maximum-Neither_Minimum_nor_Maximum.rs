/**
 * [P002733] Neither Minimum nor Maximum
 *
 * <p>给你一个整数数组 <code>nums</code> ，数组由 <strong>不同正整数</strong> 组成，请你找出并返回数组中 <strong>任一</strong> 既不是 <strong>最小值</strong> 也不是 <strong>最大值</strong> 的数字，如果不存在这样的数字，返回 <strong><code>-1</code></strong> 。</p>

<p>返回所选整数。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>nums = [3,2,1,4]
<strong>输出：</strong>2
<strong>解释：</strong>在这个示例中，最小值是 1 ，最大值是 4 。因此，2 或 3 都是有效答案。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>nums = [1,2]
<strong>输出：</strong>-1
<strong>解释：</strong>由于不存在既不是最大值也不是最小值的数字，我们无法选出满足题目给定条件的数字。因此，不存在答案，返回 -1 。
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>nums = [2,1,3]
<strong>输出：</strong>2
<strong>解释：</strong>2 既不是最小值，也不是最大值，这个示例只有这一个有效答案。 
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 100</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 100</code></li>
	<li><code>nums</code> 中的所有数字互不相同</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/neither-minimum-nor-maximum/
// discuss: https://leetcode.com/problems/neither-minimum-nor-maximum/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002733() {
    }
}