/**
 * [P001785] Minimum Elements to Add to Form a Given Sum
 *
 * <p>给你一个整数数组 <code>nums</code> ，和两个整数 <code>limit</code> 与 <code>goal</code> 。数组 <code>nums</code> 有一条重要属性：<code>abs(nums[i]) <= limit</code> 。</p>

<p>返回使数组元素总和等于 <code>goal</code> 所需要向数组中添加的 <strong>最少元素数量</strong> ，添加元素 <strong>不应改变</strong> 数组中 <code>abs(nums[i]) <= limit</code> 这一属性。</p>

<p>注意，如果 <code>x >= 0</code> ，那么 <code>abs(x)</code> 等于 <code>x</code> ；否则，等于 <code>-x</code> 。</p>

<p> </p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,-1,1], limit = 3, goal = -4
<strong>输出：</strong>2
<strong>解释：</strong>可以将 -2 和 -3 添加到数组中，数组的元素总和变为 1 - 1 + 1 - 2 - 3 = -4 。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,-10,9,1], limit = 100, goal = 0
<strong>输出：</strong>1
</pre>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 <= nums.length <= 10<sup>5</sup></code></li>
	<li><code>1 <= limit <= 10<sup>6</sup></code></li>
	<li><code>-limit <= nums[i] <= limit</code></li>
	<li><code>-10<sup>9</sup> <= goal <= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/minimum-elements-to-add-to-form-a-given-sum/
// discuss: https://leetcode.com/problems/minimum-elements-to-add-to-form-a-given-sum/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001785() {
    }
}