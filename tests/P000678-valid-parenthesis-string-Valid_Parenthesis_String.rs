/**
 * [P000678] Valid Parenthesis String
 *
 * <p>给你一个只包含三种字符的字符串，支持的字符类型分别是 <code>'('</code>、<code>')'</code> 和 <code>'*'</code>。请你检验这个字符串是否为有效字符串，如果是有效字符串返回 <code>true</code> 。</p>

<p>有效字符串符合如下规则：</p>

<ul>
	<li>任何左括号 <code>'('</code>&nbsp;必须有相应的右括号 <code>')'</code>。</li>
	<li>任何右括号 <code>')'</code>&nbsp;必须有相应的左括号 <code>'('</code>&nbsp;。</li>
	<li>左括号 <code>'('</code> 必须在对应的右括号之前 <code>')'</code>。</li>
	<li><code>'*'</code>&nbsp;可以被视为单个右括号 <code>')'</code>&nbsp;，或单个左括号 <code>'('</code>&nbsp;，或一个空字符串。</li>
	<li>一个空字符串也被视为有效字符串。</li>
</ul>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<pre>
<strong>输入：</strong>s = "()"
<strong>输出：</strong>true
</pre>

<p><strong class="example">示例 2：</strong></p>

<pre>
<strong>输入：</strong>s = "(*)"
<strong>输出：</strong>true
</pre>

<p><strong class="example">示例 3：</strong></p>

<pre>
<strong>输入：</strong>s = "(*))"
<strong>输出：</strong>true
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 100</code></li>
	<li><code>s[i]</code> 为 <code>'('</code>、<code>')'</code> 或 <code>'*'</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/valid-parenthesis-string/
// discuss: https://leetcode.com/problems/valid-parenthesis-string/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn check_valid_string(s: String) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000678() {
    }
}