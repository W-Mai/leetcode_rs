/**
 * [P002696] Minimum String Length After Removing Substrings
 *
 * <p>给你一个仅由 <strong>大写</strong> 英文字符组成的字符串 <code>s</code> 。</p>

<p>你可以对此字符串执行一些操作，在每一步操作中，你可以从 <code>s</code> 中删除 <strong>任一个</strong> <code>"AB"</code> 或 <code>"CD"</code> 子字符串。</p>

<p>通过执行操作，删除所有&nbsp;<code>"AB"</code> 和 <code>"CD"</code> 子串，返回可获得的最终字符串的 <strong>最小</strong> 可能长度。</p>

<p><strong>注意</strong>，删除子串后，重新连接出的字符串可能会产生新的&nbsp;<code>"AB"</code> 或 <code>"CD"</code> 子串。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>s = "ABFCACDB"
<strong>输出：</strong>2
<strong>解释：</strong>你可以执行下述操作：
- 从 "<em><strong>AB</strong></em>FCACDB" 中删除子串 "AB"，得到 s = "FCACDB" 。
- 从 "FCA<em><strong>CD</strong></em>B" 中删除子串 "CD"，得到 s = "FCAB" 。
- 从 "FC<strong><em>AB</em></strong>" 中删除子串 "AB"，得到 s = "FC" 。
最终字符串的长度为 2 。
可以证明 2 是可获得的最小长度。</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>s = "ACBBD"
<strong>输出：</strong>5
<strong>解释：</strong>无法执行操作，字符串长度不变。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 100</code></li>
	<li><code>s</code> 仅由大写英文字母组成</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/minimum-string-length-after-removing-substrings/
// discuss: https://leetcode.com/problems/minimum-string-length-after-removing-substrings/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn min_length(s: String) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002696() {
    }
}