/**
 * [P000472] Concatenated Words
 *
 * <p>给你一个 <strong>不含重复 </strong>单词的字符串数组 <code>words</code> ，请你找出并返回 <code>words</code> 中的所有 <strong>连接词</strong> 。</p>

<p><strong>连接词</strong> 定义为：一个完全由给定数组中的至少两个较短单词（不一定是不同的两个单词）组成的字符串。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>words = ["cat","cats","catsdogcats","dog","dogcatsdog","hippopotamuses","rat","ratcatdogcat"]
<strong>输出：</strong>["catsdogcats","dogcatsdog","ratcatdogcat"]
<strong>解释：</strong>"catsdogcats" 由 "cats", "dog" 和 "cats" 组成; 
     "dogcatsdog" 由 "dog", "cats" 和 "dog" 组成; 
     "ratcatdogcat" 由 "rat", "cat", "dog" 和 "cat" 组成。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>words = ["cat","dog","catdog"]
<strong>输出：</strong>["catdog"]</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= words[i].length &lt;= 30</code></li>
	<li><code>words[i]</code>&nbsp;仅由小写英文字母组成。&nbsp;</li>
	<li><code>words</code>&nbsp;中的所有字符串都是 <strong>唯一</strong> 的。</li>
	<li><code>1 &lt;= sum(words[i].length) &lt;= 10<sup>5</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/concatenated-words/
// discuss: https://leetcode.com/problems/concatenated-words/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000472() {
    }
}