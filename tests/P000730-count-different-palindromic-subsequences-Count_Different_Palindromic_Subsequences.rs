/**
 * [P000730] Count Different Palindromic Subsequences
 *
 * <p>给你一个字符串 <code>s</code> ，返回 <code>s</code>&nbsp;中不同的非空回文子序列个数 。由于答案可能很大，请返回对 <code>10<sup>9</sup> + 7</code> <strong>取余</strong> 的结果。</p>

<p>字符串的子序列可以经由字符串删除 0 个或多个字符获得。</p>

<p>如果一个序列与它反转后的序列一致，那么它是回文序列。</p>

<p>如果存在某个 <code>i</code> , 满足&nbsp;<code>a<sub>i</sub>&nbsp;!= b<sub>i</sub></code><sub>&nbsp;</sub>，则两个序列&nbsp;<code>a<sub>1</sub>, a<sub>2</sub>, ...</code>&nbsp;和&nbsp;<code>b<sub>1</sub>, b<sub>2</sub>, ...</code>&nbsp;不同。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>s = 'bccb'
<strong>输出：</strong>6
<strong>解释：</strong>6 个不同的非空回文子字符序列分别为：'b', 'c', 'bb', 'cc', 'bcb', 'bccb'。
注意：'bcb' 虽然出现两次但仅计数一次。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>s = 'abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba'
<strong>输出：</strong>104860361
<strong>解释：</strong>共有 3104860382 个不同的非空回文子序列，104860361 是对 10<sup>9</sup> + 7 取余后的值。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 1000</code></li>
	<li><code>s[i]</code>&nbsp;仅包含&nbsp;<code>'a'</code>,&nbsp;<code>'b'</code>,&nbsp;<code>'c'</code>&nbsp;或&nbsp;<code>'d'</code>&nbsp;</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/count-different-palindromic-subsequences/
// discuss: https://leetcode.com/problems/count-different-palindromic-subsequences/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000730() {
    }
}