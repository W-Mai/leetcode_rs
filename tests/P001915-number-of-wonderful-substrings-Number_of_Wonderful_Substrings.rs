/**
 * [P001915] Number of Wonderful Substrings
 *
 * <p>如果某个字符串中 <strong>至多一个</strong> 字母出现 <strong>奇数</strong> 次，则称其为 <strong>最美</strong> 字符串。</p>

<ul>
	<li>例如，<code>"ccjjc"</code> 和 <code>"abab"</code> 都是最美字符串，但 <code>"ab"</code> 不是。</li>
</ul>

<p>给你一个字符串 <code>word</code> ，该字符串由前十个小写英文字母组成（<code>'a'</code> 到 <code>'j'</code>）。请你返回 <code>word</code> 中 <strong>最美非空子字符串</strong> 的数目<em>。</em>如果同样的子字符串在<em> </em><code>word</code> 中出现多次，那么应当对 <strong>每次出现</strong> 分别计数<em>。</em></p>

<p><strong>子字符串</strong> 是字符串中的一个连续字符序列。</p>

<p> </p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>word = "aba"
<strong>输出：</strong>4
<strong>解释：</strong>4 个最美子字符串如下所示：
- "<strong>a</strong>ba" -> "a"
- "a<strong>b</strong>a" -> "b"
- "ab<strong>a</strong>" -> "a"
- "<strong>aba</strong>" -> "aba"
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>word = "aabb"
<strong>输出：</strong>9
<strong>解释：</strong>9 个最美子字符串如下所示：
- "<strong>a</strong>abb" -> "a"
- "<strong>aa</strong>bb" -> "aa"
- "<strong>aab</strong>b" -> "aab"
- "<strong>aabb</strong>" -> "aabb"
- "a<strong>a</strong>bb" -> "a"
- "a<strong>abb</strong>" -> "abb"
- "aa<strong>b</strong>b" -> "b"
- "aa<strong>bb</strong>" -> "bb"
- "aab<strong>b</strong>" -> "b"
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>word = "he"
<strong>输出：</strong>2
<strong>解释：</strong>2 个最美子字符串如下所示：
- "<b>h</b>e" -> "h"
- "h<strong>e</strong>" -> "e"
</pre>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 <= word.length <= 10<sup>5</sup></code></li>
	<li><code>word</code> 由从 <code>'a'</code> 到 <code>'j'</code> 的小写英文字母组成</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/number-of-wonderful-substrings/
// discuss: https://leetcode.com/problems/number-of-wonderful-substrings/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001915() {
    }
}