/**
 * [P002826] Sorting Three Groups
 *
 * <p>给你一个整数数组&nbsp;<code>nums</code>&nbsp;。<code>nums</code>&nbsp;的每个元素是 1，2 或 3。在每次操作中，你可以删除&nbsp;<code>nums</code>&nbsp;中的一个元素。返回使 nums 成为 <strong>非递减</strong>&nbsp;顺序所需操作数的 <strong>最小值</strong>。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<pre>
<b>输入：</b>nums = [2,1,3,2,1]
<b>输出：</b>3
<b>解释：</b>
其中一个最优方案是删除 nums[0]，nums[2] 和 nums[3]。
</pre>

<p><strong class="example">示例 2：</strong></p>

<pre>
<b>输入：</b>nums = [1,3,2,1,3,3]
<b>输出：</b>2
<b>解释：</b>
其中一个最优方案是删除 nums[1] 和 nums[2]。
</pre>

<p><strong class="example">示例 3：</strong></p>

<pre>
<b>输入：</b>nums = [2,2,2,2,3,3]
<b>输出：</b>0
<b>解释：</b>
nums 已是非递减顺序的。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 100</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 3</code></li>
</ul>

<p><strong>进阶：</strong>你可以使用&nbsp;<code>O(n)</code>&nbsp;时间复杂度以内的算法解决吗？</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/sorting-three-groups/
// discuss: https://leetcode.com/problems/sorting-three-groups/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002826() {
    }
}