/**
 * [P000424] Longest Repeating Character Replacement
 *
 * <p>给你一个字符串 <code>s</code> 和一个整数 <code>k</code> 。你可以选择字符串中的任一字符，并将其更改为任何其他大写英文字符。该操作最多可执行 <code>k</code> 次。</p>

<p>在执行上述操作后，返回 <em>包含相同字母的最长子字符串的长度。</em></p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>s = "ABAB", k = 2
<strong>输出：</strong>4
<strong>解释：</strong>用两个'A'替换为两个'B',反之亦然。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>s = "AABABBA", k = 1
<strong>输出：</strong>4
<strong>解释：</strong>
将中间的一个'A'替换为'B',字符串变为 "AABBBBA"。
子串 "BBBB" 有最长重复字母, 答案为 4。
可能存在其他的方法来得到同样的结果。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> 仅由大写英文字母组成</li>
	<li><code>0 &lt;= k &lt;= s.length</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/longest-repeating-character-replacement/
// discuss: https://leetcode.com/problems/longest-repeating-character-replacement/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000424() {
    }
}