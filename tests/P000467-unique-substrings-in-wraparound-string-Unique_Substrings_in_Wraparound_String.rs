/**
 * [P000467] Unique Substrings in Wraparound String
 *
 * <p>定义字符串&nbsp;<code>base</code>&nbsp;为一个&nbsp;<code>"abcdefghijklmnopqrstuvwxyz"</code>&nbsp;无限环绕的字符串，所以&nbsp;<code>base</code>&nbsp;看起来是这样的：</p>

<ul>
	<li><code>"...zabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd...."</code>.</li>
</ul>

<p>给你一个字符串&nbsp;<code>s</code> ，请你统计并返回&nbsp;<code>s</code>&nbsp;中有多少&nbsp;<strong>不同</strong><strong>非空子串</strong>&nbsp;也在&nbsp;<code>base</code>&nbsp;中出现。</p>

<p>&nbsp;</p>

<p><strong>示例&nbsp;1：</strong></p>

<pre>
<strong>输入：</strong>s = "a"
<strong>输出：</strong>1
<strong>解释：</strong>字符串 s 的子字符串 "a" 在 base 中出现。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>s = "cac"
<strong>输出：</strong>2
<strong>解释：</strong>字符串 s 有两个子字符串 ("a", "c") 在 base 中出现。
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>s = "zab"
<strong>输出：</strong>6
<strong>解释：</strong>字符串 s 有六个子字符串 ("z", "a", "b", "za", "ab", and "zab") 在 base 中出现。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><font color="#c7254e" face="Menlo, Monaco, Consolas, Courier New, monospace"><span style="font-size: 12.6px; background-color: rgb(249, 242, 244);">s</span></font> 由小写英文字母组成</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/unique-substrings-in-wraparound-string/
// discuss: https://leetcode.com/problems/unique-substrings-in-wraparound-string/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn find_substring_in_wrapround_string(s: String) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000467() {
    }
}