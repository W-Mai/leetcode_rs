/**
 * [P002768] Number of Black Blocks
 *
 * <p>给你两个整数&nbsp;<code>m</code> 和&nbsp;<code>n</code>&nbsp;，表示一个下标从 <strong>0</strong>&nbsp;开始的&nbsp;<code>m x n</code>&nbsp;的网格图。</p>

<p>给你一个下标从 <strong>0</strong>&nbsp;开始的二维整数矩阵&nbsp;<code>coordinates</code>&nbsp;，其中&nbsp;<code>coordinates[i] = [x, y]</code>&nbsp;表示坐标为&nbsp;<code>[x, y]</code>&nbsp;的格子是 <strong>黑色的</strong>&nbsp;，所有没出现在&nbsp;<code>coordinates</code>&nbsp;中的格子都是 <strong>白色的</strong>。</p>

<p>一个块定义为网格图中&nbsp;<code>2 x 2</code>&nbsp;的一个子矩阵。更正式的，对于左上角格子为 <code>[x, y]</code> 的块，其中 <code>0 &lt;= x &lt; m - 1</code> 且&nbsp;<code>0 &lt;= y &lt; n - 1</code> ，包含坐标为&nbsp;<code>[x, y]</code>&nbsp;，<code>[x + 1, y]</code>&nbsp;，<code>[x, y + 1]</code>&nbsp;和&nbsp;<code>[x + 1, y + 1]</code>&nbsp;的格子。</p>

<p>请你返回一个下标从 <strong>0</strong>&nbsp;开始长度为 <code>5</code>&nbsp;的整数数组&nbsp;<code>arr</code>&nbsp;，<code>arr[i]</code>&nbsp;表示恰好包含&nbsp;<code>i</code>&nbsp;个&nbsp;<strong>黑色</strong>&nbsp;格子的块的数目。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<b>输入：</b>m = 3, n = 3, coordinates = [[0,0]]
<b>输出：</b>[3,1,0,0,0]
<b>解释：</b>网格图如下：
<img alt="" src="https://assets.leetcode.com/uploads/2023/06/18/screen-shot-2023-06-18-at-44656-am.png" style="width: 150px; height: 128px;" />
只有 1 个块有一个黑色格子，这个块是左上角为 [0,0] 的块。
其他 3 个左上角分别为 [0,1] ，[1,0] 和 [1,1] 的块都有 0 个黑格子。
所以我们返回 [3,1,0,0,0] 。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<b>输入：</b>m = 3, n = 3, coordinates = [[0,0],[1,1],[0,2]]
<b>输出：</b>[0,2,2,0,0]
<b>解释：</b>网格图如下：
<img alt="" src="https://assets.leetcode.com/uploads/2023/06/18/screen-shot-2023-06-18-at-45018-am.png" style="width: 150px; height: 128px;" />
有 2 个块有 2 个黑色格子（左上角格子分别为 [0,0] 和 [0,1]）。
左上角为 [1,0] 和 [1,1] 的两个块，都有 1 个黑格子。
所以我们返回 [0,2,2,0,0] 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>2 &lt;= m &lt;= 10<sup>5</sup></code></li>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= coordinates.length &lt;= 10<sup>4</sup></code></li>
	<li><code>coordinates[i].length == 2</code></li>
	<li><code>0 &lt;= coordinates[i][0] &lt; m</code></li>
	<li><code>0 &lt;= coordinates[i][1] &lt; n</code></li>
	<li><code>coordinates</code>&nbsp;中的坐标对两两互不相同。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/number-of-black-blocks/
// discuss: https://leetcode.com/problems/number-of-black-blocks/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn count_black_blocks(m: i32, n: i32, coordinates: Vec<Vec<i32>>) -> Vec<i64> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002768() {
    }
}