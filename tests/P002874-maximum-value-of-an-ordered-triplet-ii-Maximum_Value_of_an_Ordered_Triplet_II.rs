/**
 * [P002874] Maximum Value of an Ordered Triplet II
 *
 * <p>给你一个下标从 <strong>0</strong> 开始的整数数组 <code>nums</code> 。</p>

<p>请你从所有满足&nbsp;<code>i &lt; j &lt; k</code> 的下标三元组 <code>(i, j, k)</code> 中，找出并返回下标三元组的最大值。如果所有满足条件的三元组的值都是负数，则返回 <code>0</code> 。</p>

<p><strong>下标三元组</strong> <code>(i, j, k)</code> 的值等于 <code>(nums[i] - nums[j]) * nums[k]</code> 。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<pre>
<strong>输入：</strong>nums = [12,6,1,2,7]
<strong>输出：</strong>77
<strong>解释：</strong>下标三元组 (0, 2, 4) 的值是 (nums[0] - nums[2]) * nums[4] = 77 。
可以证明不存在值大于 77 的有序下标三元组。
</pre>

<p><strong class="example">示例 2：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,10,3,4,19]
<strong>输出：</strong>133
<strong>解释：</strong>下标三元组 (1, 2, 4) 的值是 (nums[1] - nums[2]) * nums[4] = 133 。
可以证明不存在值大于 133 的有序下标三元组。 
</pre>

<p><strong class="example">示例 3：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,2,3]
<strong>输出：</strong>0
<strong>解释：</strong>唯一的下标三元组 (0, 1, 2) 的值是一个负数，(nums[0] - nums[1]) * nums[2] = -3 。因此，答案是 0 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>3 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-ii/
// discuss: https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-ii/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002874() {
    }
}