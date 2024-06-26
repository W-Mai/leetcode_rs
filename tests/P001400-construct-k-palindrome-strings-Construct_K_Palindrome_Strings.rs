/**
 * [P001400] Construct K Palindrome Strings
 *
 * <p>给你一个字符串 <code>s</code>&nbsp;和一个整数 <code>k</code>&nbsp;。请你用 <code>s</code>&nbsp;字符串中 <strong>所有字符</strong>&nbsp;构造 <code>k</code>&nbsp;个非空 <strong>回文串</strong>&nbsp;。</p>

<p>如果你可以用&nbsp;<code>s</code>&nbsp;中所有字符构造&nbsp;<code>k</code>&nbsp;个回文字符串，那么请你返回 <strong>True</strong>&nbsp;，否则返回&nbsp;<strong>False</strong>&nbsp;。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>s = &quot;annabelle&quot;, k = 2
<strong>输出：</strong>true
<strong>解释：</strong>可以用 s 中所有字符构造 2 个回文字符串。
一些可行的构造方案包括：&quot;anna&quot; + &quot;elble&quot;，&quot;anbna&quot; + &quot;elle&quot;，&quot;anellena&quot; + &quot;b&quot;
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>s = &quot;leetcode&quot;, k = 3
<strong>输出：</strong>false
<strong>解释：</strong>无法用 s 中所有字符构造 3 个回文串。
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>s = &quot;true&quot;, k = 4
<strong>输出：</strong>true
<strong>解释：</strong>唯一可行的方案是让 s 中每个字符单独构成一个字符串。
</pre>

<p><strong>示例 4：</strong></p>

<pre>
<strong>输入：</strong>s = &quot;yzyzyzyzyzyzyzy&quot;, k = 2
<strong>输出：</strong>true
<strong>解释：</strong>你只需要将所有的 z 放在一个字符串中，所有的 y 放在另一个字符串中。那么两个字符串都是回文串。
</pre>

<p><strong>示例 5：</strong></p>

<pre>
<strong>输入：</strong>s = &quot;cr&quot;, k = 7
<strong>输出：</strong>false
<strong>解释：</strong>我们没有足够的字符去构造 7 个回文串。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10^5</code></li>
	<li><code>s</code>&nbsp;中所有字符都是小写英文字母。</li>
	<li><code>1 &lt;= k &lt;= 10^5</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/construct-k-palindrome-strings/
// discuss: https://leetcode.com/problems/construct-k-palindrome-strings/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001400() {
    }
}