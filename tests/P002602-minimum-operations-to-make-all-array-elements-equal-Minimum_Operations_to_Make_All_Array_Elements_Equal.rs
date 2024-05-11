/**
 * [P002602] Minimum Operations to Make All Array Elements Equal
 *
 * <p>给你一个正整数数组&nbsp;<code>nums</code>&nbsp;。</p>

<p>同时给你一个长度为 <code>m</code>&nbsp;的整数数组&nbsp;<code>queries</code>&nbsp;。第 <code>i</code>&nbsp;个查询中，你需要将 <code>nums</code>&nbsp;中所有元素变成&nbsp;<code>queries[i]</code>&nbsp;。你可以执行以下操作&nbsp;<strong>任意</strong>&nbsp;次：</p>

<ul>
	<li>将数组里一个元素&nbsp;<strong>增大</strong>&nbsp;或者&nbsp;<strong>减小</strong>&nbsp;<code>1</code>&nbsp;。</li>
</ul>

<p>请你返回一个长度为 <code>m</code>&nbsp;的数组<em>&nbsp;</em><code>answer</code>&nbsp;，其中<em>&nbsp;</em><code>answer[i]</code>是将&nbsp;<code>nums</code>&nbsp;中所有元素变成&nbsp;<code>queries[i]</code>&nbsp;的&nbsp;<strong>最少</strong>&nbsp;操作次数。</p>

<p><strong>注意</strong>，每次查询后，数组变回最开始的值。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><b>输入：</b>nums = [3,1,6,8], queries = [1,5]
<b>输出：</b>[14,10]
<b>解释：</b>第一个查询，我们可以执行以下操作：
- 将 nums[0] 减小 2 次，nums = [1,1,6,8] 。
- 将 nums[2] 减小 5 次，nums = [1,1,1,8] 。
- 将 nums[3] 减小 7 次，nums = [1,1,1,1] 。
第一个查询的总操作次数为 2 + 5 + 7 = 14 。
第二个查询，我们可以执行以下操作：
- 将 nums[0] 增大 2 次，nums = [5,1,6,8] 。
- 将 nums[1] 增大 4 次，nums = [5,5,6,8] 。
- 将 nums[2] 减小 1 次，nums = [5,5,5,8] 。
- 将 nums[3] 减小 3 次，nums = [5,5,5,5] 。
第二个查询的总操作次数为 2 + 4 + 1 + 3 = 10 。
</pre>

<p><strong>示例 2：</strong></p>

<pre><b>输入：</b>nums = [2,9,6,3], queries = [10]
<b>输出：</b>[20]
<b>解释：</b>我们可以将数组中所有元素都增大到 10 ，总操作次数为 8 + 1 + 4 + 7 = 20 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>n == nums.length</code></li>
	<li><code>m == queries.length</code></li>
	<li><code>1 &lt;= n, m &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i], queries[i] &lt;= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/minimum-operations-to-make-all-array-elements-equal/
// discuss: https://leetcode.com/problems/minimum-operations-to-make-all-array-elements-equal/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn min_operations(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002602() {
    }
}