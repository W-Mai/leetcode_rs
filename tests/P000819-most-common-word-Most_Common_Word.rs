/**
 * [P000819] Most Common Word
 *
 * <p>给你一个字符串 <code>paragraph</code> 和一个表示禁用词的字符串数组 <code>banned</code> ，返回出现频率最高的非禁用词。题目数据 <strong>保证 </strong>至少存在一个非禁用词，且答案<strong> 唯一 </strong>。</p>

<p><code>paragraph</code> 中的单词 <strong>不区分大小写</strong> ，答案应以 <strong>小写 </strong>形式返回。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<pre>
<strong>输入：</strong>paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.", banned = ["hit"]
<strong>输出：</strong>"ball"
<strong>解释：</strong>
"hit" 出现了 3 次，但它是禁用词。
"ball" 出现了两次（没有其他单词出现这么多次），因此它是段落中出现频率最高的非禁用词。
请注意，段落中的单词不区分大小写，
标点符号会被忽略（即使它们紧挨着单词，如 "ball,"），
并且尽管 "hit" 出现的次数更多，但它不能作为答案，因为它是禁用词。
</pre>

<p><strong class="example">示例 2：</strong></p>

<pre>
<strong>输入：</strong>paragraph = "a.", banned = []
<strong>输出：</strong>"a"
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= paragraph.length &lt;= 1000</code></li>
	<li><code>paragraph</code> 由英文字母、空格 <code>' '</code>、和以下符号组成：<code>"!?',;."</code></li>
	<li><code>0 &lt;= banned.length &lt;= 100</code></li>
	<li><code>1 &lt;= banned[i].length &lt;= 10</code></li>
	<li><code>banned[i]</code> 仅由小写英文字母组成</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/most-common-word/
// discuss: https://leetcode.com/problems/most-common-word/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000819() {
    }
}