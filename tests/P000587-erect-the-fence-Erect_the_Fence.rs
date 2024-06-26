/**
 * [P000587] Erect the Fence
 *
 * <p>给定一个数组 <code>trees</code>，其中 <code>trees[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> 表示树在花园中的位置。</p>

<p>你被要求用最短长度的绳子把整个花园围起来，因为绳子很贵。只有把&nbsp;<strong>所有的树都围起来</strong>，花园才围得很好。</p>

<p>返回<em>恰好位于围栏周边的树木的坐标</em>。</p>

<p><strong>示例 1:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2021/04/24/erect2-plane.jpg" style="width: 400px;" /></p>

<pre>
<strong>输入:</strong> points = [[1,1],[2,2],[2,0],[2,4],[3,3],[4,2]]
<strong>输出:</strong> [[1,1],[2,0],[3,3],[2,4],[4,2]]</pre>

<p><strong>示例 2:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2021/04/24/erect1-plane.jpg" style="height: 393px; width: 400px;" /></p>

<pre>
<strong>输入:</strong> points = [[1,2],[2,2],[4,2]]
<strong>输出:</strong> [[4,2],[2,2],[1,2]]</pre>

<p>&nbsp;</p>

<p><strong>注意:</strong></p>

<ul>
	<li><code>1 &lt;= points.length &lt;= 3000</code></li>
	<li><code>points[i].length == 2</code></li>
	<li><code>0 &lt;= x<sub>i</sub>, y<sub>i</sub>&nbsp;&lt;= 100</code></li>
	<li>
	<p data-group="1-1">所有给定的点都是&nbsp;<strong>唯一&nbsp;</strong>的。</p>
	</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/erect-the-fence/
// discuss: https://leetcode.com/problems/erect-the-fence/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000587() {
    }
}