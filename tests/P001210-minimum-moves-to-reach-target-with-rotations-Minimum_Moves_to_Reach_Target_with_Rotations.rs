/**
 * [P001210] Minimum Moves to Reach Target with Rotations
 *
 * <p>你还记得那条风靡全球的贪吃蛇吗？</p>

<p>我们在一个&nbsp;<code>n*n</code>&nbsp;的网格上构建了新的迷宫地图，蛇的长度为 2，也就是说它会占去两个单元格。蛇会从左上角（<code>(0, 0)</code>&nbsp;和&nbsp;<code>(0, 1)</code>）开始移动。我们用 <code>0</code> 表示空单元格，用 1 表示障碍物。蛇需要移动到迷宫的右下角（<code>(n-1, n-2)</code>&nbsp;和&nbsp;<code>(n-1, n-1)</code>）。</p>

<p>每次移动，蛇可以这样走：</p>

<ul>
	<li>如果没有障碍，则向右移动一个单元格。并仍然保持身体的水平／竖直状态。</li>
	<li>如果没有障碍，则向下移动一个单元格。并仍然保持身体的水平／竖直状态。</li>
	<li>如果它处于水平状态并且其下面的两个单元都是空的，就顺时针旋转 90 度。蛇从（<code>(r, c)</code>、<code>(r, c+1)</code>）移动到 （<code>(r, c)</code>、<code>(r+1, c)</code>）。<br>
	<img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/09/28/image-2.png" style="height: 134px; width: 300px;"></li>
	<li>如果它处于竖直状态并且其右面的两个单元都是空的，就逆时针旋转 90 度。蛇从（<code>(r, c)</code>、<code>(r+1, c)</code>）移动到（<code>(r, c)</code>、<code>(r, c+1)</code>）。<br>
	<img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/09/28/image-1.png" style="height: 121px; width: 300px;"></li>
</ul>

<p>返回蛇抵达目的地所需的最少移动次数。</p>

<p>如果无法到达目的地，请返回&nbsp;<code>-1</code>。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<p><strong><img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/09/28/image.png" style="height: 439px; width: 400px;"></strong></p>

<pre><strong>输入：</strong>grid = [[0,0,0,0,0,1],
               [1,1,0,0,1,0],
&nbsp;              [0,0,0,0,1,1],
&nbsp;              [0,0,1,0,1,0],
&nbsp;              [0,1,1,0,0,0],
&nbsp;              [0,1,1,0,0,0]]
<strong>输出：</strong>11
<strong>解释：
</strong>一种可能的解决方案是 [右, 右, 顺时针旋转, 右, 下, 下, 下, 下, 逆时针旋转, 右, 下]。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>grid = [[0,0,1,1,1,1],
&nbsp;              [0,0,0,0,1,1],
&nbsp;              [1,1,0,0,0,1],
&nbsp;              [1,1,1,0,0,1],
&nbsp;              [1,1,1,0,0,1],
&nbsp;              [1,1,1,0,0,0]]
<strong>输出：</strong>9
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 100</code></li>
	<li><code>0 &lt;= grid[i][j] &lt;= 1</code></li>
	<li>蛇保证从空单元格开始出发。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/minimum-moves-to-reach-target-with-rotations/
// discuss: https://leetcode.com/problems/minimum-moves-to-reach-target-with-rotations/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001210() {
    }
}