/**
 * [P000140] Word Break II
 *
 * <p>给定一个字符串 <code>s</code> 和一个字符串字典<meta charset="UTF-8" />&nbsp;<code>wordDict</code>&nbsp;，在字符串<meta charset="UTF-8" />&nbsp;<code>s</code>&nbsp;中增加空格来构建一个句子，使得句子中所有的单词都在词典中。<strong>以任意顺序</strong> 返回所有这些可能的句子。</p>

<p><strong>注意：</strong>词典中的同一个单词可能在分段中被重复使用多次。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<pre>
<strong>输入:</strong>s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
<strong>输出:</strong>["cats and dog","cat sand dog"]
</pre>

<p><strong class="example">示例 2：</strong></p>

<pre>
<strong>输入:</strong>s = "pineapplepenapple", wordDict = ["apple","pen","applepen","pine","pineapple"]
<strong>输出:</strong>["pine apple pen apple","pineapple pen apple","pine applepen apple"]
<strong>解释:</strong> 注意你可以重复使用字典中的单词。
</pre>

<p><strong class="example">示例&nbsp;3：</strong></p>

<pre>
<strong>输入:</strong>s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
<strong>输出:</strong>[]
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<p><meta charset="UTF-8" /></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 20</code></li>
	<li><code>1 &lt;= wordDict.length &lt;= 1000</code></li>
	<li><code>1 &lt;= wordDict[i].length &lt;= 10</code></li>
	<li><code>s</code>&nbsp;和&nbsp;<code>wordDict[i]</code>&nbsp;仅有小写英文字母组成</li>
	<li><code>wordDict</code>&nbsp;中所有字符串都 <strong>不同</strong></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/word-break-ii/
// discuss: https://leetcode.com/problems/word-break-ii/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000140() {
    }
}