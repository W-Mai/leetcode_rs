/**
 * [P000692] Top K Frequent Words
 *
 * <p>给定一个单词列表&nbsp;<code>words</code>&nbsp;和一个整数 <code>k</code> ，返回前&nbsp;<code>k</code><em>&nbsp;</em>个出现次数最多的单词。</p>

<p>返回的答案应该按单词出现频率由高到低排序。如果不同的单词有相同出现频率， <strong>按字典顺序</strong> 排序。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入:</strong> words = ["i", "love", "leetcode", "i", "love", "coding"], k = 2
<strong>输出:</strong> ["i", "love"]
<strong>解析:</strong> "i" 和 "love" 为出现次数最多的两个单词，均为2次。
    注意，按字母顺序 "i" 在 "love" 之前。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入:</strong> ["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"], k = 4
<strong>输出:</strong> ["the", "is", "sunny", "day"]
<strong>解析:</strong> "the", "is", "sunny" 和 "day" 是出现次数最多的四个单词，
    出现次数依次为 4, 3, 2 和 1 次。
</pre>

<p>&nbsp;</p>

<p><strong>注意：</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 500</code></li>
	<li><code>1 &lt;= words[i] &lt;= 10</code></li>
	<li><code>words[i]</code>&nbsp;由小写英文字母组成。</li>
	<li><code>k</code> 的取值范围是&nbsp;<code>[1, <strong>不同</strong> words[i] 的数量]</code></li>
</ul>

<p>&nbsp;</p>

<p><strong>进阶：</strong>尝试以&nbsp;<code>O(n log k)</code> 时间复杂度和&nbsp;<code>O(n)</code> 空间复杂度解决。</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/top-k-frequent-words/
// discuss: https://leetcode.com/problems/top-k-frequent-words/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000692() {
    }
}