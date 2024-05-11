/**
 * [P002862] Maximum Element-Sum of a Complete Subset of Indices
 *
 * <p>给你一个下标从 <strong>1</strong> 开始、由 <code>n</code> 个整数组成的数组。你需要从&nbsp;<code>nums</code>&nbsp;选择一个&nbsp;<strong>完全集</strong>，其中每对元素下标的乘积都是一个 <span data-keyword="perfect-square">完全平方数</span>，例如选择&nbsp;<code>a<sub>i</sub></code>&nbsp;和&nbsp;<code>a<sub>j</sub></code>&nbsp;，<code>i * j</code>&nbsp;一定是完全平方数。</p>

<p>返回&nbsp;<strong>完全子集</strong> 所能取到的 <strong>最大元素和</strong> 。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>nums = [8,7,3,5,7,2,4,9]
<strong>输出：</strong>16
<strong>解释：</strong>我们选择了下标 1 和 4 的元素，并且 1 * 4 是一个完全平方数。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>nums = [8,10,3,8,1,13,7,9,4]
<strong>输出：</strong>20
<strong>解释：</strong>我们选择了下标 1，4 和 9 的元素。1 * 4，1 * 9，4 * 9 都是完全平方数。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/maximum-element-sum-of-a-complete-subset-of-indices/
// discuss: https://leetcode.com/problems/maximum-element-sum-of-a-complete-subset-of-indices/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i64 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002862() {
    }
}