/**
 * [P000306] Additive Number
 *
 * <p><strong>累加数</strong> 是一个字符串，组成它的数字可以形成累加序列。</p>

<p>一个有效的 <strong>累加序列</strong> 必须<strong> 至少 </strong>包含 3 个数。除了最开始的两个数以外，序列中的每个后续数字必须是它之前两个数字之和。</p>

<p>给你一个只包含数字&nbsp;<code>'0'-'9'</code>&nbsp;的字符串，编写一个算法来判断给定输入是否是 <strong>累加数</strong> 。如果是，返回 <code>true</code> ；否则，返回 <code>false</code> 。</p>

<p><strong>说明：</strong>累加序列里的数，除数字 0 之外，<strong>不会</strong> 以 0 开头，所以不会出现&nbsp;<code>1, 2, 03</code> 或者&nbsp;<code>1, 02, 3</code>&nbsp;的情况。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong><code>"112358"</code>
<strong>输出：</strong>true 
<strong>解释：</strong>累加序列为: <code>1, 1, 2, 3, 5, 8 </code>。1 + 1 = 2, 1 + 2 = 3, 2 + 3 = 5, 3 + 5 = 8
</pre>

<p><strong>示例&nbsp;2：</strong></p>

<pre>
<strong>输入<code>：</code></strong><code>"199100199"</code>
<strong>输出：</strong>true 
<strong>解释：</strong>累加序列为: <code>1, 99, 100, 199。</code>1 + 99 = 100, 99 + 100 = 199</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= num.length &lt;= 35</code></li>
	<li><code>num</code> 仅由数字（<code>0</code> - <code>9</code>）组成</li>
</ul>

<p>&nbsp;</p>

<p><strong>进阶：</strong>你计划如何处理由过大的整数输入导致的溢出?</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/additive-number/
// discuss: https://leetcode.com/problems/additive-number/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn is_additive_number(num: String) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000306() {
    }
}