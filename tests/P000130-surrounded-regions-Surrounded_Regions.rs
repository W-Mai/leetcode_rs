/**
 * [P000130] Surrounded Regions
 *
 * 给你一个 <code>m x n</code> 的矩阵 <code>board</code> ，由若干字符 <code>'X'</code> 和 <code>'O'</code> ，找到所有被 <code>'X'</code> 围绕的区域，并将这些区域里所有的 <code>'O'</code> 用 <code>'X'</code> 填充。
<div class="original__bRMd">
<div>
<p> </p>

<p><strong>示例 1：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/xogrid.jpg" style="width: 550px; height: 237px;" />
<pre>
<strong>输入：</strong>board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
<strong>输出：</strong>[["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
<strong>解释：</strong>被围绕的区间不会存在于边界上，换句话说，任何边界上的 <code>'O'</code> 都不会被填充为 <code>'X'</code>。 任何不在边界上，或不与边界上的 <code>'O'</code> 相连的 <code>'O'</code> 最终都会被填充为 <code>'X'</code>。如果两个元素在水平或垂直方向相邻，则称它们是“相连”的。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>board = [["X"]]
<strong>输出：</strong>[["X"]]
</pre>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>m == board.length</code></li>
	<li><code>n == board[i].length</code></li>
	<li><code>1 <= m, n <= 200</code></li>
	<li><code>board[i][j]</code> 为 <code>'X'</code> 或 <code>'O'</code></li>
</ul>
</div>
</div>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/surrounded-regions/
// discuss: https://leetcode.com/problems/surrounded-regions/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000130() {
    }
}