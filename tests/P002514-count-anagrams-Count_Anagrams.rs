/**
 * [P002514] Count Anagrams
 *
 * <p>给你一个字符串&nbsp;<code>s</code>&nbsp;，它包含一个或者多个单词。单词之间用单个空格&nbsp;<code>' '</code>&nbsp;隔开。</p>

<p>如果字符串 <code>t</code>&nbsp;中第 <code>i</code>&nbsp;个单词是 <code>s</code>&nbsp;中第 <code>i</code>&nbsp;个单词的一个&nbsp;<strong>排列</strong>&nbsp;，那么我们称字符串&nbsp;<code>t</code>&nbsp;是字符串&nbsp;<code>s</code>&nbsp;的同位异构字符串。</p>

<ul>
	<li>比方说，<code>"acb dfe"</code>&nbsp;是&nbsp;<code>"abc def"</code>&nbsp;的同位异构字符串，但是&nbsp;<code>"def cab"</code>&nbsp;和&nbsp;<code>"adc bef"</code>&nbsp;不是。</li>
</ul>

<p>请你返回<em>&nbsp;</em><code>s</code>&nbsp;的同位异构字符串的数目，由于答案可能很大，请你将它对&nbsp;<code>10<sup>9</sup> + 7</code>&nbsp;<strong>取余</strong> 后返回。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><b>输入：</b>s = "too hot"
<b>输出：</b>18
<b>解释：</b>输入字符串的一些同位异构字符串为 "too hot" ，"oot hot" ，"oto toh" ，"too toh" 以及 "too oht" 。
</pre>

<p><strong>示例 2：</strong></p>

<pre><b>输入：</b>s = "aa"
<b>输出：</b>1
<strong>解释：</strong>输入字符串只有一个同位异构字符串。</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> 只包含小写英文字母和空格&nbsp;<code>' '</code>&nbsp;。</li>
	<li>相邻单词之间由单个空格隔开。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/count-anagrams/
// discuss: https://leetcode.com/problems/count-anagrams/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn count_anagrams(s: String) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002514() {
    }
}