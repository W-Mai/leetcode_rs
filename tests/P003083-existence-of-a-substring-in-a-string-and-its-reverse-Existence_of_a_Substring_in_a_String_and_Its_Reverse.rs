/**
 * [P003083] Existence of a Substring in a String and Its Reverse
 *
 * <p>给你一个字符串 <code>s</code> ，请你判断字符串 <code>s</code> 是否存在一个长度为 <code>2</code> 的子字符串，在其反转后的字符串中也出现。</p>

<p>如果存在这样的子字符串，返回 <code>true</code>；如果不存在，返回 <code>false</code> 。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<div class="example-block" style="border-color: var(--border-tertiary); border-left-width: 2px; color: var(--text-secondary); font-size: .875rem; margin-bottom: 1rem; margin-top: 1rem; overflow: visible; padding-left: 1rem;">
<p><strong>输入：</strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">s = "leetcode"</span></p>

<p><strong>输出：</strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">true</span></p>

<p><strong>解释：</strong>子字符串 <code>"ee"</code> 的长度为 <code>2</code>，它也出现在 <code>reverse(s) == "edocteel"</code> 中。</p>
</div>

<p><strong class="example">示例 2：</strong></p>

<div class="example-block" style="border-color: var(--border-tertiary); border-left-width: 2px; color: var(--text-secondary); font-size: .875rem; margin-bottom: 1rem; margin-top: 1rem; overflow: visible; padding-left: 1rem;">
<p><strong>输入：</strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">s = "abcba"</span></p>

<p><strong>输出：</strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">true</span></p>

<p><strong>解释：</strong>所有长度为 <code>2</code> 的子字符串 <code>"ab"</code>、<code>"bc"</code>、<code>"cb"</code>、<code>"ba"</code> 也都出现在 <code>reverse(s) == "abcba"</code> 中。</p>
</div>

<p><strong class="example">示例 3：</strong></p>

<div class="example-block" style="border-color: var(--border-tertiary); border-left-width: 2px; color: var(--text-secondary); font-size: .875rem; margin-bottom: 1rem; margin-top: 1rem; overflow: visible; padding-left: 1rem;">
<p><strong>输入：</strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">s = "abcd"</span></p>

<p><strong>输出：</strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">false</span></p>

<p><strong>解释：</strong>字符串 <code>s</code> 中不存在满足「在其反转后的字符串中也出现」且长度为 <code>2</code> 的子字符串。</p>
</div>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 100</code></li>
	<li>字符串 <code>s</code> 仅由小写英文字母组成。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/existence-of-a-substring-in-a-string-and-its-reverse/
// discuss: https://leetcode.com/problems/existence-of-a-substring-in-a-string-and-its-reverse/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn is_substring_present(s: String) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P003083() {
    }
}