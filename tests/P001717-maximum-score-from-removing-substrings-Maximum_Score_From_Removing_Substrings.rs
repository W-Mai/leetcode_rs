/**
 * [P001717] Maximum Score From Removing Substrings
 *
 * <p>给你一个字符串 <code>s</code> 和两个整数 <code>x</code> 和 <code>y</code> 。你可以执行下面两种操作任意次。</p>

<ul>
	<li>删除子字符串 <code>"ab"</code> 并得到 <code>x</code> 分。

	<ul>
		<li>比方说，从 <code>"c<strong>ab</strong>xbae"</code> 删除 <code>ab</code> ，得到 <code>"cxbae"</code> 。</li>
	</ul>
	</li>
	<li>删除子字符串<code>"ba"</code> 并得到 <code>y</code> 分。
	<ul>
		<li>比方说，从 <code>"cabx<strong>ba</strong>e"</code> 删除 <code>ba</code> ，得到 <code>"cabxe"</code> 。</li>
	</ul>
	</li>
</ul>

<p>请返回对 <code>s</code> 字符串执行上面操作若干次能得到的最大得分。</p>

<p> </p>

<p><strong>示例 1：</strong></p>

<pre><b>输入：</b>s = "cdbcbbaaabab", x = 4, y = 5
<b>输出：</b>19
<strong>解释：</strong>
- 删除 "cdbcbbaaa<strong>ba</strong>b" 中加粗的 "ba" ，得到 s = "cdbcbbaaab" ，加 5 分。
- 删除 "cdbcbbaa<strong>ab</strong>" 中加粗的 "ab" ，得到 s = "cdbcbbaa" ，加 4 分。
- 删除 "cdbcb<strong>ba</strong>a" 中加粗的 "ba" ，得到 s = "cdbcba" ，加 5 分。
- 删除 "cdbc<strong>ba</strong>" 中加粗的 "ba" ，得到 s = "cdbc" ，加 5 分。
总得分为 5 + 4 + 5 + 5 = 19 。</pre>

<p><strong>示例 2：</strong></p>

<pre><b>输入：</b>s = "aabbaaxybbaabb", x = 5, y = 4
<b>输出：</b>20
</pre>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= x, y &lt;= 10<sup>4</sup></code></li>
	<li><code>s</code> 只包含小写英文字母。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/maximum-score-from-removing-substrings/
// discuss: https://leetcode.com/problems/maximum-score-from-removing-substrings/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001717() {
    }
}