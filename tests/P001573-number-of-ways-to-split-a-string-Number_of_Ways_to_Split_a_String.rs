/**
 * [P001573] Number of Ways to Split a String
 *
 * <p>给你一个二进制串&nbsp;<code>s</code>&nbsp; （一个只包含 0 和 1 的字符串），我们可以将 <code>s</code>&nbsp;分割成 3 个 <strong>非空</strong>&nbsp;字符串 s1, s2, s3 （s1 + s2 + s3 = s）。</p>

<p>请你返回分割&nbsp;<code>s</code>&nbsp;的方案数，满足 s1，s2 和 s3 中字符 &#39;1&#39; 的数目相同。</p>

<p>由于答案可能很大，请将它对 10^9 + 7 取余后返回。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>s = &quot;10101&quot;
<strong>输出：</strong>4
<strong>解释：</strong>总共有 4 种方法将 s 分割成含有 &#39;1&#39; 数目相同的三个子字符串。
&quot;1|010|1&quot;
&quot;1|01|01&quot;
&quot;10|10|1&quot;
&quot;10|1|01&quot;
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>s = &quot;1001&quot;
<strong>输出：</strong>0
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>s = &quot;0000&quot;
<strong>输出：</strong>3
<strong>解释：</strong>总共有 3 种分割 s 的方法。
&quot;0|0|00&quot;
&quot;0|00|0&quot;
&quot;00|0|0&quot;
</pre>

<p><strong>示例 4：</strong></p>

<pre><strong>输入：</strong>s = &quot;100100010100110&quot;
<strong>输出：</strong>12
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>s[i] == &#39;0&#39;</code>&nbsp;或者&nbsp;<code>s[i] == &#39;1&#39;</code></li>
	<li><code>3 &lt;= s.length &lt;= 10^5</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/number-of-ways-to-split-a-string/
// discuss: https://leetcode.com/problems/number-of-ways-to-split-a-string/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn num_ways(s: String) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001573() {
    }
}