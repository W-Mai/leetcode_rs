/**
 * [P001608] Special Array With X Elements Greater Than or Equal X
 *
 * <p>给你一个非负整数数组 <code>nums</code> 。如果存在一个数 <code>x</code> ，使得 <code>nums</code> 中恰好有 <code>x</code> 个元素 <strong>大于或者等于</strong> <code>x</code> ，那么就称 <code>nums</code> 是一个 <strong>特殊数组</strong> ，而 <code>x</code> 是该数组的 <strong>特征值</strong> 。</p>

<p>注意： <code>x</code> <strong>不必</strong> 是 <code>nums</code> 的中的元素。</p>

<p>如果数组 <code>nums</code> 是一个 <strong>特殊数组</strong> ，请返回它的特征值 <code>x</code> 。否则，返回<em> </em><code>-1</code> 。可以证明的是，如果 <code>nums</code> 是特殊数组，那么其特征值 <code>x</code> 是 <strong>唯一的</strong> 。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>nums = [3,5]
<strong>输出：</strong>2
<strong>解释：</strong>有 2 个元素（3 和 5）大于或等于 2 。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>nums = [0,0]
<strong>输出：</strong>-1
<strong>解释：</strong>没有满足题目要求的特殊数组，故而也不存在特征值 x 。
如果 x = 0，应该有 0 个元素 &gt;= x，但实际有 2 个。
如果 x = 1，应该有 1 个元素 &gt;= x，但实际有 0 个。
如果 x = 2，应该有 2 个元素 &gt;= x，但实际有 0 个。
x 不能取更大的值，因为 nums 中只有两个元素。</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>nums = [0,4,3,0,4]
<strong>输出：</strong>3
<strong>解释：</strong>有 3 个元素大于或等于 3 。
</pre>

<p><strong>示例 4：</strong></p>

<pre><strong>输入：</strong>nums = [3,6,7,7,0]
<strong>输出：</strong>-1
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 100</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 1000</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x/
// discuss: https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001608() {
    }
}