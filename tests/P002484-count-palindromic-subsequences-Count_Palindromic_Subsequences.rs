/**
 * [P002484] Count Palindromic Subsequences
 *
 * <p>给你数字字符串&nbsp;<code>s</code>&nbsp;，请你返回&nbsp;<code>s</code>&nbsp;中长度为&nbsp;<code>5</code>&nbsp;的 <b>回文子序列</b>&nbsp;数目。由于答案可能很大，请你将答案对&nbsp;<code>10<sup>9</sup> + 7</code>&nbsp;<strong>取余</strong>&nbsp;后返回。</p>

<p><strong>提示：</strong></p>

<ul>
	<li>如果一个字符串从前往后和从后往前读相同，那么它是 <strong>回文字符串</strong>&nbsp;。</li>
	<li>子序列是一个字符串中删除若干个字符后，不改变字符顺序，剩余字符构成的字符串。</li>
</ul>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>s = "103301"
<b>输出：</b>2
<b>解释：</b>
总共有 6 长度为 5 的子序列："10330" ，"10331" ，"10301" ，"10301" ，"13301" ，"03301" 。
它们中有两个（都是 "10301"）是回文的。
</pre>

<p><strong>示例 2：</strong></p>

<pre><b>输入：</b>s = "0000000"
<b>输出：</b>21
<b>解释：</b>所有 21 个长度为 5 的子序列都是 "00000" ，都是回文的。
</pre>

<p><strong>示例 3：</strong></p>

<pre><b>输入：</b>s = "9999900000"
<b>输出：</b>2
<b>解释：</b>仅有的两个回文子序列是 "99999" 和 "00000" 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>4</sup></code></li>
	<li><code>s</code>&nbsp;只包含数字字符。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/count-palindromic-subsequences/
// discuss: https://leetcode.com/problems/count-palindromic-subsequences/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn count_palindromes(s: String) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002484() {
    }
}