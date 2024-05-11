/**
 * [P002553] Separate the Digits in an Array
 *
 * <p>给你一个正整数数组&nbsp;<code>nums</code>&nbsp;，请你返回一个数组<em>&nbsp;</em><code>answer</code> ，你需要将&nbsp;<code>nums</code>&nbsp;中每个整数进行数位分割后，按照&nbsp;<code>nums</code>&nbsp;中出现的&nbsp;<strong>相同顺序</strong>&nbsp;放入答案数组中。</p>

<p>对一个整数进行数位分割，指的是将整数各个数位按原本出现的顺序排列成数组。</p>

<ul>
	<li>比方说，整数&nbsp;<code>10921</code>&nbsp;，分割它的各个数位得到&nbsp;<code>[1,0,9,2,1]</code>&nbsp;。</li>
</ul>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><b>输入：</b>nums = [13,25,83,77]
<b>输出：</b>[1,3,2,5,8,3,7,7]
<b>解释：</b>
- 分割 13 得到 [1,3] 。
- 分割 25 得到 [2,5] 。
- 分割 83 得到 [8,3] 。
- 分割 77 得到 [7,7] 。
answer = [1,3,2,5,8,3,7,7] 。answer 中的数字分割结果按照原数字在数组中的相同顺序排列。
</pre>

<p><strong>示例 2：</strong></p>

<pre><b>输入：</b>nums = [7,1,3,9]
<b>输出：</b>[7,1,3,9]
<b>解释：</b>nums 中每个整数的分割是它自己。
answer = [7,1,3,9] 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/separate-the-digits-in-an-array/
// discuss: https://leetcode.com/problems/separate-the-digits-in-an-array/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002553() {
    }
}