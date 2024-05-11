/**
 * [P001330] Reverse Subarray To Maximize Array Value
 *
 * <p>给你一个整数数组&nbsp;<code>nums</code> 。「数组值」定义为所有满足&nbsp;<code>0 &lt;= i &lt; nums.length-1</code>&nbsp;的&nbsp;<code>|nums[i]-nums[i+1]|</code>&nbsp;的和。</p>

<p>你可以选择给定数组的任意子数组，并将该子数组翻转。但你只能执行这个操作&nbsp;<strong>一次</strong> 。</p>

<p>请你找到可行的最大 <strong>数组值&nbsp;</strong>。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>nums = [2,3,1,5,4]
<strong>输出：</strong>10
<strong>解释：</strong>通过翻转子数组 [3,1,5] ，数组变成 [2,5,1,3,4] ，数组值为 10 。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>nums = [2,4,9,24,2,1,10]
<strong>输出：</strong>68
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 3*10^4</code></li>
	<li><code>-10^5 &lt;= nums[i] &lt;= 10^5</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/reverse-subarray-to-maximize-array-value/
// discuss: https://leetcode.com/problems/reverse-subarray-to-maximize-array-value/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001330() {
    }
}