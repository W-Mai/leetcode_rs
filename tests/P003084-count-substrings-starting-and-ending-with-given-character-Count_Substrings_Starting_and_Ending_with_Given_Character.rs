/**
 * [P003084] Count Substrings Starting and Ending with Given Character
 *
 * <p>给你一个字符串 <code>s</code> 和一个字符 <code>c </code>。返回在字符串 <code>s</code> 中并且以 <code>c</code> 字符开头和结尾的<span data-keyword="substring-nonempty">非空子字符串</span>的总数。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<div class="example-block" style="border-color: var(--border-tertiary); border-left-width: 2px; color: var(--text-secondary); font-size: .875rem; margin-bottom: 1rem; margin-top: 1rem; overflow: visible; padding-left: 1rem;">
<p><strong>输入：</strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">s = "abada", c = "a"</span></p>

<p><strong>输出：</strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">6</span></p>

<p><strong>解释：</strong>以 <code>"a"</code> 开头和结尾的子字符串有： <code>"<strong><u>a</u></strong>bada"</code>、<code>"<u><strong>aba</strong></u>da"</code>、<code>"<u><strong>abada</strong></u>"</code>、<code>"ab<u><strong>a</strong></u>da"</code>、<code>"ab<u><strong>ada</strong></u>"</code>、<code>"abad<u><strong>a</strong></u>"</code>。</p>
</div>

<p><strong class="example">示例 2：</strong></p>

<div class="example-block" style="border-color: var(--border-tertiary); border-left-width: 2px; color: var(--text-secondary); font-size: .875rem; margin-bottom: 1rem; margin-top: 1rem; overflow: visible; padding-left: 1rem;">
<p><strong>输入：</strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">s = "zzz", c = "z"</span></p>

<p><strong>输出：</strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">6</span></p>

<p><strong>解释：</strong>字符串 <code>s</code> 中总共有 <code>6</code> 个子字符串，并且它们都以 <code>"z"</code> 开头和结尾。</p>
</div>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> 和 <code>c</code> 均由小写英文字母组成。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/count-substrings-starting-and-ending-with-given-character/
// discuss: https://leetcode.com/problems/count-substrings-starting-and-ending-with-given-character/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn count_substrings(s: String, c: char) -> i64 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P003084() {
    }
}