/**
 * [P001415] The k-th Lexicographical String of All Happy Strings of Length n
 *
 * <p>一个 「开心字符串」定义为：</p>

<ul>
	<li>仅包含小写字母&nbsp;<code>[&#39;a&#39;, &#39;b&#39;, &#39;c&#39;]</code>.</li>
	<li>对所有在&nbsp;<code>1</code>&nbsp;到&nbsp;<code>s.length - 1</code>&nbsp;之间的&nbsp;<code>i</code>&nbsp;，满足&nbsp;<code>s[i] != s[i + 1]</code>&nbsp;（字符串的下标从 1 开始）。</li>
</ul>

<p>比方说，字符串&nbsp;<strong>&quot;abc&quot;</strong>，<strong>&quot;ac&quot;，&quot;b&quot;</strong> 和&nbsp;<strong>&quot;abcbabcbcb&quot;</strong>&nbsp;都是开心字符串，但是&nbsp;<strong>&quot;aa&quot;</strong>，<strong>&quot;baa&quot;</strong>&nbsp;和&nbsp;<strong>&quot;ababbc&quot;</strong>&nbsp;都不是开心字符串。</p>

<p>给你两个整数 <code>n</code>&nbsp;和 <code>k</code>&nbsp;，你需要将长度为 <code>n</code>&nbsp;的所有开心字符串按字典序排序。</p>

<p>请你返回排序后的第 k 个开心字符串，如果长度为 <code>n</code>&nbsp;的开心字符串少于 <code>k</code>&nbsp;个，那么请你返回 <strong>空字符串</strong>&nbsp;。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>n = 1, k = 3
<strong>输出：</strong>&quot;c&quot;
<strong>解释：</strong>列表 [&quot;a&quot;, &quot;b&quot;, &quot;c&quot;] 包含了所有长度为 1 的开心字符串。按照字典序排序后第三个字符串为 &quot;c&quot; 。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>n = 1, k = 4
<strong>输出：</strong>&quot;&quot;
<strong>解释：</strong>长度为 1 的开心字符串只有 3 个。
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>n = 3, k = 9
<strong>输出：</strong>&quot;cab&quot;
<strong>解释：</strong>长度为 3 的开心字符串总共有 12 个 [&quot;aba&quot;, &quot;abc&quot;, &quot;aca&quot;, &quot;acb&quot;, &quot;bab&quot;, &quot;bac&quot;, &quot;bca&quot;, &quot;bcb&quot;, &quot;cab&quot;, &quot;cac&quot;, &quot;cba&quot;, &quot;cbc&quot;] 。第 9 个字符串为 &quot;cab&quot;
</pre>

<p><strong>示例 4：</strong></p>

<pre><strong>输入：</strong>n = 2, k = 7
<strong>输出：</strong>&quot;&quot;
</pre>

<p><strong>示例 5：</strong></p>

<pre><strong>输入：</strong>n = 10, k = 100
<strong>输出：</strong>&quot;abacbabacb&quot;
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10</code></li>
	<li><code>1 &lt;= k &lt;= 100</code></li>
</ul>

<p>&nbsp;</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/
// discuss: https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001415() {
    }
}