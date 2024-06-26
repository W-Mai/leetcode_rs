/**
 * [P002662] Minimum Cost of a Path With Special Roads
 *
 * <p>给你一个数组 <code>start</code> ，其中 <code>start = [startX, startY]</code> 表示你的初始位置位于二维空间上的 <code>(startX, startY)</code> 。另给你一个数组 <code>target</code> ，其中 <code>target = [targetX, targetY]</code> 表示你的目标位置 <code>(targetX, targetY)</code> 。</p>

<p>从位置 <code>(x1, y1)</code> 到空间中任一其他位置 <code>(x2, y2)</code> 的代价是 <code>|x2 - x1| + |y2 - y1|</code> 。</p>

<p>给你一个二维数组 <code>specialRoads</code> ，表示空间中存在的一些特殊路径。其中 <code>specialRoads[i] = [x1<sub>i</sub>, y1<sub>i</sub>, x2<sub>i</sub>, y2<sub>i</sub>, cost<sub>i</sub>]</code> 表示第 <code>i</code> 条特殊路径可以从 <code>(x1<sub>i</sub>, y1<sub>i</sub>)</code> 到 <code>(x2<sub>i</sub>, y2<sub>i</sub>)</code> ，但成本等于 <code>cost<sub>i</sub></code> 。你可以使用每条特殊路径任意次数。</p>

<p>返回从 <code>(startX, startY)</code> 到 <code>(targetX, targetY)</code> 所需的最小代价。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>start = [1,1], target = [4,5], specialRoads = [[1,2,3,3,2],[3,4,4,5,1]]
<strong>输出：</strong>5
<strong>解释：</strong>从 (1,1) 到 (4,5) 的最优路径如下：
- (1,1) -&gt; (1,2) ，移动的代价是 |1 - 1| + |2 - 1| = 1 。
- (1,2) -&gt; (3,3) ，移动使用第一条特殊路径，代价是 2 。
- (3,3) -&gt; (3,4) ，移动的代价是 |3 - 3| + |4 - 3| = 1.
- (3,4) -&gt; (4,5) ，移动使用第二条特殊路径，代价是 1 。
总代价是 1 + 2 + 1 + 1 = 5 。
可以证明无法以小于 5 的代价完成从 (1,1) 到 (4,5) 。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>start = [3,2], target = [5,7], specialRoads = [[3,2,3,4,4],[3,3,5,5,5],[3,4,5,6,6]]
<strong>输出：</strong>7
<strong>解释：</strong>最优路径是不使用任何特殊路径，直接以 |5 - 3| + |7 - 2| = 7 的代价从初始位置到达目标位置。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>start.length == target.length == 2</code></li>
	<li><code>1 &lt;= startX &lt;= targetX &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= startY &lt;= targetY &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= specialRoads.length &lt;= 200</code></li>
	<li><code>specialRoads[i].length == 5</code></li>
	<li><code>startX &lt;= x1<sub>i</sub>, x2<sub>i</sub> &lt;= targetX</code></li>
	<li><code>startY &lt;= y1<sub>i</sub>, y2<sub>i</sub> &lt;= targetY</code></li>
	<li><code>1 &lt;= cost<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/minimum-cost-of-a-path-with-special-roads/
// discuss: https://leetcode.com/problems/minimum-cost-of-a-path-with-special-roads/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn minimum_cost(start: Vec<i32>, target: Vec<i32>, special_roads: Vec<Vec<i32>>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002662() {
    }
}