/**
 * [P002745] Construct the Longest New String
 *
 * <p>给你三个整数&nbsp;<code>x</code>&nbsp;，<code>y</code>&nbsp;和&nbsp;<code>z</code>&nbsp;。</p>

<p>这三个整数表示你有&nbsp;<code>x</code>&nbsp;个&nbsp;<code>"AA"</code>&nbsp;字符串，<code>y</code>&nbsp;个&nbsp;<code>"BB"</code>&nbsp;字符串，和&nbsp;<code>z</code>&nbsp;个&nbsp;<code>"AB"</code>&nbsp;字符串。你需要选择这些字符串中的部分字符串（可以全部选择也可以一个都不选择），将它们按顺序连接得到一个新的字符串。新字符串不能包含子字符串&nbsp;<code>"AAA"</code>&nbsp;或者&nbsp;<code>"BBB"</code>&nbsp;。</p>

<p>请你返回 <em>新字符串的最大可能长度。</em></p>

<p><strong>子字符串</strong>&nbsp;是一个字符串中一段连续 <strong>非空</strong>&nbsp;的字符序列。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<pre>
<b>输入：</b>x = 2, y = 5, z = 1
<b>输出：</b>12
<strong>解释： </strong>我们可以按顺序连接 "BB" ，"AA" ，"BB" ，"AA" ，"BB" 和 "AB" ，得到新字符串 "BBAABBAABBAB" 。
字符串长度为 12 ，无法得到一个更长的符合题目要求的字符串。
</pre>

<p><strong class="example">示例 2：</strong></p>

<pre>
<b>输入：</b>x = 3, y = 2, z = 2
<b>输出：</b>14
<b>解释：</b>我们可以按顺序连接 "AB" ，"AB" ，"AA" ，"BB" ，"AA" ，"BB" 和 "AA" ，得到新字符串 "ABABAABBAABBAA" 。
字符串长度为 14 ，无法得到一个更长的符合题目要求的字符串。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= x, y, z &lt;= 50</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/construct-the-longest-new-string/
// discuss: https://leetcode.com/problems/construct-the-longest-new-string/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn longest_string(x: i32, y: i32, z: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002745() {
    }
}