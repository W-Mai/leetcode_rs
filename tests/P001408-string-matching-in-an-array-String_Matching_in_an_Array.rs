/**
 * [P001408] String Matching in an Array
 *
 * <p>给你一个字符串数组 <code>words</code> ，数组中的每个字符串都可以看作是一个单词。请你按 <strong>任意</strong> 顺序返回 <code>words</code> 中是其他单词的子字符串的所有单词。</p>

<p>如果你可以删除 <code>words[j]</code>&nbsp;最左侧和/或最右侧的若干字符得到 <code>words[i]</code> ，那么字符串 <code>words[i]</code> 就是 <code>words[j]</code> 的一个子字符串。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>words = ["mass","as","hero","superhero"]
<strong>输出：</strong>["as","hero"]
<strong>解释：</strong>"as" 是 "mass" 的子字符串，"hero" 是 "superhero" 的子字符串。
["hero","as"] 也是有效的答案。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>words = ["leetcode","et","code"]
<strong>输出：</strong>["et","code"]
<strong>解释：</strong>"et" 和 "code" 都是 "leetcode" 的子字符串。
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>words = ["blue","green","bu"]
<strong>输出：</strong>[]
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 100</code></li>
	<li><code>1 &lt;= words[i].length &lt;= 30</code></li>
	<li><code>words[i]</code> 仅包含小写英文字母。</li>
	<li>题目数据 <strong>保证</strong> 每个 <code>words[i]</code> 都是独一无二的。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/string-matching-in-an-array/
// discuss: https://leetcode.com/problems/string-matching-in-an-array/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001408() {
    }
}