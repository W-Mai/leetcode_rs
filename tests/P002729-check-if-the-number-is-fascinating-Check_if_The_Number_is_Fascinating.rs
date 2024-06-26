/**
 * [P002729] Check if The Number is Fascinating
 *
 * <p>给你一个三位数整数 <code>n</code>&nbsp;。</p>

<p>如果经过以下修改得到的数字 <strong>恰好</strong>&nbsp;包含数字 <code>1</code>&nbsp;到 <code>9</code>&nbsp;各一次且不包含任何 <code>0</code>&nbsp;，那么我们称数字 <code>n</code>&nbsp;是 <strong>迷人的</strong>&nbsp;：</p>

<ul>
	<li>将&nbsp;<code>n</code>&nbsp;与数字&nbsp;<code>2 * n</code> 和&nbsp;<code>3 * n</code>&nbsp;<strong>连接</strong>&nbsp;。</li>
</ul>

<p>如果 <code>n</code>&nbsp;是迷人的，返回&nbsp;<code>true</code>，否则返回&nbsp;<code>false</code>&nbsp;。</p>

<p><strong>连接</strong>&nbsp;两个数字表示把它们首尾相接连在一起。比方说&nbsp;<code>121</code> 和&nbsp;<code>371</code>&nbsp;连接得到&nbsp;<code>121371</code>&nbsp;。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><b>输入：</b>n = 192
<b>输出：</b>true
<b>解释：</b>我们将数字 n = 192 ，2 * n = 384 和 3 * n = 576 连接，得到 192384576 。这个数字包含 1 到 9 恰好各一次。
</pre>

<p><strong>示例 2：</strong></p>

<pre><b>输入：</b>n = 100
<b>输出：</b>false
<b>解释：</b>我们将数字 n = 100 ，2 * n = 200 和 3 * n = 300 连接，得到 100200300 。这个数字不符合上述条件。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>100 &lt;= n &lt;= 999</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/check-if-the-number-is-fascinating/
// discuss: https://leetcode.com/problems/check-if-the-number-is-fascinating/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn is_fascinating(n: i32) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002729() {
    }
}