/**
 * [P002781] Length of the Longest Valid Substring
 *
 * <p>给你一个字符串&nbsp;<code>word</code>&nbsp;和一个字符串数组&nbsp;<code>forbidden</code>&nbsp;。</p>

<p>如果一个字符串不包含&nbsp;<code>forbidden</code>&nbsp;中的任何字符串，我们称这个字符串是&nbsp;<strong>合法</strong>&nbsp;的。</p>

<p>请你返回字符串 <code>word</code>&nbsp;的一个 <strong>最长合法子字符串</strong>&nbsp;的长度。</p>

<p><strong>子字符串</strong> 指的是一个字符串中一段连续的字符，它可以为空。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<b>输入：</b>word = "cbaaaabc", forbidden = ["aaa","cb"]
<b>输出：</b>4
<b>解释：</b>总共有 11 个合法子字符串："c", "b", "a", "ba", "aa", "bc", "baa", "aab", "ab", "abc" 和 "aabc"。最长合法子字符串的长度为 4 。
其他子字符串都要么包含 "aaa" ，要么包含 "cb" 。</pre>

<p><strong>示例 2：</strong></p>

<pre>
<b>输入：</b>word = "leetcode", forbidden = ["de","le","e"]
<strong>输出：</strong>4
<b>解释：</b>总共有 11 个合法子字符串："l" ，"t" ，"c" ，"o" ，"d" ，"tc" ，"co" ，"od" ，"tco" ，"cod" 和 "tcod" 。最长合法子字符串的长度为 4 。
所有其他子字符串都至少包含 "de" ，"le" 和 "e" 之一。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= word.length &lt;= 10<sup>5</sup></code></li>
	<li><code>word</code>&nbsp;只包含小写英文字母。</li>
	<li><code>1 &lt;= forbidden.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= forbidden[i].length &lt;= 10</code></li>
	<li><code>forbidden[i]</code>&nbsp;只包含小写英文字母。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/length-of-the-longest-valid-substring/
// discuss: https://leetcode.com/problems/length-of-the-longest-valid-substring/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn longest_valid_substring(word: String, forbidden: Vec<String>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002781() {
    }
}