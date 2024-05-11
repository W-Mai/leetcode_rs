/**
 * [P001300] Sum of Mutated Array Closest to Target
 *
 * <p>给你一个整数数组&nbsp;<code>arr</code> 和一个目标值&nbsp;<code>target</code> ，请你返回一个整数&nbsp;<code>value</code>&nbsp;，使得将数组中所有大于&nbsp;<code>value</code> 的值变成&nbsp;<code>value</code> 后，数组的和最接近&nbsp; <code>target</code>&nbsp;（最接近表示两者之差的绝对值最小）。</p>

<p>如果有多种使得和最接近&nbsp;<code>target</code>&nbsp;的方案，请你返回这些整数中的最小值。</p>

<p>请注意，答案不一定是&nbsp;<code>arr</code> 中的数字。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>arr = [4,9,3], target = 10
<strong>输出：</strong>3
<strong>解释：</strong>当选择 value 为 3 时，数组会变成 [3, 3, 3]，和为 9 ，这是最接近 target 的方案。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>arr = [2,3,5], target = 10
<strong>输出：</strong>5
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>arr = [60864,25176,27249,21296,20204], target = 56803
<strong>输出：</strong>11361
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 10^4</code></li>
	<li><code>1 &lt;= arr[i], target &lt;= 10^5</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/sum-of-mutated-array-closest-to-target/
// discuss: https://leetcode.com/problems/sum-of-mutated-array-closest-to-target/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001300() {
    }
}