/**
 * [P002765] Longest Alternating Subarray
 *
 * <p>给你一个下标从 <strong>0</strong>&nbsp;开始的整数数组&nbsp;<code>nums</code>&nbsp;。如果 <code>nums</code>&nbsp;中长度为&nbsp;<code>m</code>&nbsp;的子数组&nbsp;<code>s</code>&nbsp;满足以下条件，我们称它是一个 <strong>交替子数组</strong> ：</p>

<ul>
	<li><code>m</code>&nbsp;大于&nbsp;<code>1</code>&nbsp;。</li>
	<li><code>s<sub>1</sub> = s<sub>0</sub> + 1</code>&nbsp;。</li>
	<li>下标从 <strong>0</strong> 开始的子数组&nbsp;<code>s</code>&nbsp;与数组&nbsp;<code>[s<sub>0</sub>, s<sub>1</sub>, s<sub>0</sub>, s<sub>1</sub>,...,s<sub>(m-1) % 2</sub>]</code>&nbsp;一样。也就是说，<code>s<sub>1</sub> - s<sub>0</sub> = 1</code>&nbsp;，<code>s<sub>2</sub> - s<sub>1</sub> = -1</code>&nbsp;，<code>s<sub>3</sub> - s<sub>2</sub> = 1</code>&nbsp;，<code>s<sub>4</sub> - s<sub>3</sub> = -1</code>&nbsp;，以此类推，直到&nbsp;<code>s[m - 1] - s[m - 2] = (-1)<sup>m</sup></code>&nbsp;。</li>
</ul>

<p>请你返回 <code>nums</code>&nbsp;中所有 <strong>交替</strong>&nbsp;子数组中，最长的长度，如果不存在交替子数组，请你返回 <code>-1</code>&nbsp;。</p>

<p>子数组是一个数组中一段连续 <strong>非空</strong>&nbsp;的元素序列。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<b>输入：</b>nums = [2,3,4,3,4]
<b>输出：</b>4
<b>解释：</b>交替子数组有 [3,4] ，[3,4,3] 和 [3,4,3,4] 。最长的子数组为 [3,4,3,4] ，长度为4 。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<b>输入：</b>nums = [4,5,6]
<b>输出：</b>2
<strong>解释：</strong>[4,5] 和 [5,6] 是仅有的两个交替子数组。它们长度都为 2 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 100</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/longest-alternating-subarray/
// discuss: https://leetcode.com/problems/longest-alternating-subarray/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002765() {
    }
}