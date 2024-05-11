/**
 * [LCR 137] 模糊搜索验证
 *
 * <p>请设计一个程序来支持用户在文本编辑器中的模糊搜索功能。用户输入内容中可能使用到如下两种通配符：</p>

<ul>
	<li><code>'.'</code> 匹配任意单个字符。</li>
	<li><code>'*'</code> 匹配零个或多个前面的那一个元素。</li>
</ul>

<p>&nbsp;</p>

<p>请返回用户输入内容 <code>input</code> 所有字符是否可以匹配原文字符串 <code>article</code>。</p>

<p>&nbsp;</p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入: </strong>article = "aa", input = "a"
<strong>输出:</strong> false
<strong>解释:</strong> "a" 无法匹配 "aa" 整个字符串。
</pre>

<p><strong>示例 2:</strong></p>

<pre>
<strong>输入: </strong>article = "aa", input = "a*"
<strong>输出:</strong> true
<strong>解释:</strong>&nbsp;因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
</pre>

<p><strong>示例&nbsp;3:</strong></p>

<pre>
<strong>输入: </strong>article = "ab", input = ".*"
<strong>输出:</strong> true
<strong>解释:</strong>&nbsp;".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= article.length &lt;= 20</code></li>
	<li><code>1 &lt;= input.length &lt;= 20</code></li>
	<li><code>article</code> 只包含从 <code>a-z</code> 的小写字母。</li>
	<li><code>input</code> 只包含从 <code>a-z</code> 的小写字母，以及字符 <code>.</code> 和 <code>*</code> 。</li>
	<li>保证每次出现字符 <code>*</code> 时，前面都匹配到有效的字符</li>
</ul>

<p>&nbsp;</p>

<p>注意：本题与主站 10&nbsp;题相同：<a href="https://leetcode-cn.com/problems/regular-expression-matching/">https://leetcode-cn.com/problems/regular-expression-matching/</a></p>

<p>&nbsp;</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/zheng-ze-biao-da-shi-pi-pei-lcof/
// discuss: https://leetcode.com/problems/zheng-ze-biao-da-shi-pi-pei-lcof/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn article_match(s: String, p: String) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCR 137() {
    }
}