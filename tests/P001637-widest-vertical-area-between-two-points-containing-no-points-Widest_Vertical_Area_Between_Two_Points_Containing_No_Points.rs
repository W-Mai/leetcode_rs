/**
 * [P001637] Widest Vertical Area Between Two Points Containing No Points
 *
 * <p>给你&nbsp;<code>n</code>&nbsp;个二维平面上的点 <code>points</code> ，其中&nbsp;<code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code>&nbsp;，请你返回两点之间内部不包含任何点的&nbsp;<strong>最宽垂直区域</strong> 的宽度。</p>

<p><strong>垂直区域</strong> 的定义是固定宽度，而 y 轴上无限延伸的一块区域（也就是高度为无穷大）。 <strong>最宽垂直区域</strong> 为宽度最大的一个垂直区域。</p>

<p>请注意，垂直区域&nbsp;<strong>边上</strong>&nbsp;的点&nbsp;<strong>不在</strong>&nbsp;区域内。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>
<img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/10/31/points3.png" style="width: 276px; height: 371px;" />​
<pre>
<b>输入：</b>points = [[8,7],[9,9],[7,4],[9,7]]
<b>输出：</b>1
<b>解释：</b>红色区域和蓝色区域都是最优区域。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<b>输入：</b>points = [[3,1],[9,0],[1,0],[1,4],[5,3],[8,8]]
<b>输出：</b>3
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>n == points.length</code></li>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>points[i].length == 2</code></li>
	<li><code>0 &lt;= x<sub>i</sub>, y<sub>i</sub>&nbsp;&lt;= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/
// discuss: https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001637() {
    }
}