/**
 * [P002851] String Transformation
 *
 * <p>给你两个长度都为 <code>n</code>&nbsp;的字符串&nbsp;<code>s</code> 和&nbsp;<code>t</code>&nbsp;。你可以对字符串 <code>s</code>&nbsp;执行以下操作：</p>

<ul>
	<li>将 <code>s</code>&nbsp;长度为 <code>l</code>&nbsp;（<code>0 &lt; l &lt; n</code>）的 <strong>后缀字符串</strong>&nbsp;删除，并将它添加在 <code>s</code>&nbsp;的开头。<br />
	比方说，<code>s = 'abcd'</code>&nbsp;，那么一次操作中，你可以删除后缀&nbsp;<code>'cd'</code>&nbsp;，并将它添加到&nbsp;<code>s</code>&nbsp;的开头，得到&nbsp;<code>s = 'cdab'</code>&nbsp;。</li>
</ul>

<p>给你一个整数&nbsp;<code>k</code>&nbsp;，请你返回&nbsp;<strong>恰好</strong> <code>k</code>&nbsp;次操作将<em>&nbsp;</em><code>s</code> 变为<em>&nbsp;</em><code>t</code>&nbsp;的方案数。</p>

<p>由于答案可能很大，返回答案对&nbsp;<code>10<sup>9</sup> + 7</code>&nbsp;<strong>取余</strong>&nbsp;后的结果。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<pre>
<b>输入：</b>s = "abcd", t = "cdab", k = 2
<b>输出：</b>2
<b>解释：</b>
第一种方案：
第一次操作，选择 index = 3 开始的后缀，得到 s = "dabc" 。
第二次操作，选择 index = 3 开始的后缀，得到 s = "cdab" 。

第二种方案：
第一次操作，选择 index = 1 开始的后缀，得到 s = "bcda" 。
第二次操作，选择 index = 1 开始的后缀，得到 s = "cdab" 。
</pre>

<p><strong class="example">示例 2：</strong></p>

<pre>
<b>输入：</b>s = "ababab", t = "ababab", k = 1
<b>输出：</b>2
<b>解释：</b>
第一种方案：
选择 index = 2 开始的后缀，得到 s = "ababab" 。

第二种方案：
选择 index = 4 开始的后缀，得到 s = "ababab" 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>2 &lt;= s.length &lt;= 5 * 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>15</sup></code></li>
	<li><code>s.length == t.length</code></li>
	<li><code>s</code> 和&nbsp;<code>t</code>&nbsp;都只包含小写英文字母。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/string-transformation/
// discuss: https://leetcode.com/problems/string-transformation/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn number_of_ways(s: String, t: String, k: i64) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002851() {
    }
}