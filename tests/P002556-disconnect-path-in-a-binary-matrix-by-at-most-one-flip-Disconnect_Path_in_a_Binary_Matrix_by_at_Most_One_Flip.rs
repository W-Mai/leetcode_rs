/**
 * [P002556] Disconnect Path in a Binary Matrix by at Most One Flip
 *
 * <p>给你一个下标从 <strong>0</strong>&nbsp;开始的&nbsp;<code>m x n</code>&nbsp;<strong>二进制</strong> 矩阵&nbsp;<code>grid</code>&nbsp;。你可以从一个格子&nbsp;<code>(row, col)</code>&nbsp;移动到格子&nbsp;<code>(row + 1, col)</code>&nbsp;或者&nbsp;<code>(row, col + 1)</code>&nbsp;，前提是前往的格子值为 <code>1</code>&nbsp;。如果从&nbsp;<code>(0, 0)</code>&nbsp;到&nbsp;<code>(m - 1, n - 1)</code>&nbsp;没有任何路径，我们称该矩阵是&nbsp;<strong>不连通</strong>&nbsp;的。</p>

<p>你可以翻转 <strong>最多一个</strong>&nbsp;格子的值（也可以不翻转）。你 <strong>不能翻转</strong>&nbsp;格子&nbsp;<code>(0, 0)</code> 和&nbsp;<code>(m - 1, n - 1)</code>&nbsp;。</p>

<p>如果可以使矩阵不连通，请你返回&nbsp;<code>true</code>&nbsp;，否则返回<em>&nbsp;</em><code>false</code><em>&nbsp;</em>。</p>

<p><strong>注意</strong>&nbsp;，翻转一个格子的值，可以使它的值从&nbsp;<code>0</code>&nbsp;变&nbsp;<code>1</code>&nbsp;，或从&nbsp;<code>1</code>&nbsp;变&nbsp;<code>0</code>&nbsp;。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2022/12/07/yetgrid2drawio.png" style="width: 441px; height: 151px;" /></p>

<pre>
<b>输入：</b>grid = [[1,1,1],[1,0,0],[1,1,1]]
<strong>输出：</strong>true
<b>解释：</b>按照上图所示我们翻转蓝色格子里的值，翻转后从 (0, 0) 到 (2, 2) 没有路径。
</pre>

<p><strong>示例 2：</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2022/12/07/yetgrid3drawio.png" /></p>

<pre>
<b>输入：</b>grid = [[1,1,1],[1,0,1],[1,1,1]]
<b>输出：</b>false
<b>解释：</b>无法翻转至多一个格子，使 (0, 0) 到 (2, 2) 没有路径。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 1000</code></li>
	<li><code>1 &lt;= m * n &lt;= 10<sup>5</sup></code></li>
	<li><code>grid[0][0] == grid[m - 1][n - 1] == 1</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/disconnect-path-in-a-binary-matrix-by-at-most-one-flip/
// discuss: https://leetcode.com/problems/disconnect-path-in-a-binary-matrix-by-at-most-one-flip/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn is_possible_to_cut_path(grid: Vec<Vec<i32>>) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002556() {
    }
}