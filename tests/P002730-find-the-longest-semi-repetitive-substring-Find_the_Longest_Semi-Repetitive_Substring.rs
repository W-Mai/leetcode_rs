/**
 * [P002730] Find the Longest Semi-Repetitive Substring
 *
 * <p>给你一个下标从 <strong>0</strong>&nbsp;开始的字符串&nbsp;<code>s</code>&nbsp;，这个字符串只包含&nbsp;<code>0</code>&nbsp;到&nbsp;<code>9</code>&nbsp;的数字字符。</p>

<p>如果一个字符串&nbsp;<code>t</code>&nbsp;中至多有一对相邻字符是相等的，那么称这个字符串 <code>t</code> 是 <strong>半重复的</strong>&nbsp;。例如，<code>0010</code> 、<code>002020</code> 、<code>0123</code> 、<code>2002</code> 和 <code>54944</code> 是半重复字符串，而 <code>00101022</code> 和 <code>1101234883</code> 不是。</p>

<p>请你返回 <code>s</code>&nbsp;中最长 <strong>半重复</strong>&nbsp;子字符串的长度。</p>

<p>一个 <strong>子字符串</strong>&nbsp;是一个字符串中一段连续 <strong>非空</strong>&nbsp;的字符。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<b>输入：</b>s = "52233"
<b>输出：</b>4
<b>解释：</b>最长半重复子字符串是 "5223" ，子字符串从 i = 0 开始，在 j = 3 处结束。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<b>输入：</b>s = "5494"
<b>输出：</b>4
<b>解释：</b>s 就是一个半重复字符串，所以答案为 4 。
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<b>输入：</b>s = "1111111"
<b>输出：</b>2
<b>解释：</b>最长半重复子字符串是 "11" ，子字符串从 i = 0 开始，在 j = 1 处结束。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 50</code></li>
	<li><code>'0' &lt;= s[i] &lt;= '9'</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/find-the-longest-semi-repetitive-substring/
// discuss: https://leetcode.com/problems/find-the-longest-semi-repetitive-substring/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002730() {
    }
}