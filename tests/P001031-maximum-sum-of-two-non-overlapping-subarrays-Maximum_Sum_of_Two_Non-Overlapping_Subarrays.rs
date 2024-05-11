/**
 * [P001031] Maximum Sum of Two Non-Overlapping Subarrays
 *
 * <p>给你一个整数数组 <code>nums</code> 和两个整数 <code>firstLen</code> 和 <code>secondLen</code>，请你找出并返回两个非重叠<strong> 子数组 </strong>中元素的最大和<em>，</em>长度分别为 <code>firstLen</code> 和 <code>secondLen</code> 。</p>

<p>长度为 <code>firstLen</code> 的子数组可以出现在长为 <code>secondLen</code> 的子数组之前或之后，但二者必须是不重叠的。</p>

<p>子数组是数组的一个 <strong>连续</strong> 部分。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>nums = [0,6,5,2,2,5,1,9,4], firstLen = 1, secondLen = 2
<strong>输出：</strong>20
<strong>解释：</strong>子数组的一种选择中，[9] 长度为 1，[6,5] 长度为 2。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>nums = [3,8,1,3,2,1,8,9,0], firstLen = 3, secondLen = 2
<strong>输出：</strong>29
<strong>解释：</strong>子数组的一种选择中，[3,8,1] 长度为 3，[8,9] 长度为 2。
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>nums = [2,1,5,6,0,9,5,0,3,8], firstLen = 4, secondLen = 3
<strong>输出：</strong>31
<strong>解释：</strong>子数组的一种选择中，[5,6,0,9] 长度为 4，[0,3,8] 长度为 3。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= firstLen, secondLen &lt;= 1000</code></li>
	<li><code>2 &lt;= firstLen + secondLen &lt;= 1000</code></li>
	<li><code>firstLen + secondLen &lt;= nums.length &lt;= 1000</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 1000</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/maximum-sum-of-two-non-overlapping-subarrays/
// discuss: https://leetcode.com/problems/maximum-sum-of-two-non-overlapping-subarrays/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001031() {
    }
}