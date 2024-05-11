/**
 * [P002639] Find the Width of Columns of a Grid
 *
 * <p>给你一个下标从 <strong>0</strong>&nbsp;开始的&nbsp;<code>m x n</code>&nbsp;整数矩阵&nbsp;<code>grid</code>&nbsp;。矩阵中某一列的宽度是这一列数字的最大 <strong>字符串长度</strong>&nbsp;。</p>

<ul>
	<li>比方说，如果&nbsp;<code>grid = [[-10], [3], [12]]</code>&nbsp;，那么唯一一列的宽度是&nbsp;<code>3</code>&nbsp;，因为&nbsp;<code>-10</code>&nbsp;的字符串长度为&nbsp;<code>3</code>&nbsp;。</li>
</ul>

<p>请你返回一个大小为 <code>n</code>&nbsp;的整数数组&nbsp;<code>ans</code>&nbsp;，其中&nbsp;<code>ans[i]</code>&nbsp;是第&nbsp;<code>i</code>&nbsp;列的宽度。</p>

<p>一个有 <code>len</code>&nbsp;个数位的整数 <code>x</code>&nbsp;，如果是非负数，那么&nbsp;<strong>字符串</strong><strong>长度</strong>&nbsp;为&nbsp;<code>len</code>&nbsp;，否则为&nbsp;<code>len + 1</code>&nbsp;。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><b>输入：</b>grid = [[1],[22],[333]]
<b>输出：</b>[3]
<b>解释：</b>第 0 列中，333 字符串长度为 3 。
</pre>

<p><strong>示例 2：</strong></p>

<pre><b>输入：</b>grid = [[-15,1,3],[15,7,12],[5,6,-2]]
<b>输出：</b>[3,1,2]
<b>解释：</b>
第 0 列中，只有 -15 字符串长度为 3 。
第 1 列中，所有整数的字符串长度都是 1 。
第 2 列中，12 和 -2 的字符串长度都为 2 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 100 </code></li>
	<li><code>-10<sup>9</sup> &lt;= grid[r][c] &lt;= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/find-the-width-of-columns-of-a-grid/
// discuss: https://leetcode.com/problems/find-the-width-of-columns-of-a-grid/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002639() {
    }
}