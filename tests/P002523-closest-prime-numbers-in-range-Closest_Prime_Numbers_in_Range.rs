/**
 * [P002523] Closest Prime Numbers in Range
 *
 * <p>给你两个正整数&nbsp;<code>left</code> 和&nbsp;<code>right</code>&nbsp;，请你找到两个整数&nbsp;<code>num1</code> 和&nbsp;<code>num2</code>&nbsp;，它们满足：</p>

<ul>
	<li><code>left &lt;= nums1 &lt; nums2 &lt;= right&nbsp;</code>&nbsp;。</li>
	<li><code>nums1</code> 和&nbsp;<code>nums2</code>&nbsp;都是 <strong>质数</strong>&nbsp;。</li>
	<li><code>nums2 - nums1</code>&nbsp;是满足上述条件的质数对中的 <strong>最小值</strong>&nbsp;。</li>
</ul>

<p>请你返回正整数数组&nbsp;<code>ans = [nums1, nums2]</code>&nbsp;。如果有多个整数对满足上述条件，请你返回&nbsp;<code>nums1</code>&nbsp;最小的质数对。如果不存在符合题意的质数对，请你返回&nbsp;<code>[-1, -1]</code>&nbsp;。</p>

<p>如果一个整数大于&nbsp;<code>1</code>&nbsp;，且只能被&nbsp;<code>1</code> 和它自己整除，那么它是一个 <strong>质数</strong>。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<b>输入：</b>left = 10, right = 19
<b>输出：</b>[11,13]
<b>解释：</b>10 到 19 之间的质数为 11 ，13 ，17 和 19 。
质数对的最小差值是 2 ，[11,13] 和 [17,19] 都可以得到最小差值。
由于 11 比 17 小，我们返回第一个质数对。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<b>输入：</b>left = 4, right = 6
<b>输出：</b>[-1,-1]
<b>解释：</b>给定范围内只有一个质数，所以题目条件无法被满足。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= left &lt;= right &lt;= 10<sup>6</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/closest-prime-numbers-in-range/
// discuss: https://leetcode.com/problems/closest-prime-numbers-in-range/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002523() {
    }
}