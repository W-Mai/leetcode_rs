/**
 * [P000335] Self Crossing
 *
 * <p>给你一个整数数组 <code>distance</code><em> </em>。</p>

<p>从 <strong>X-Y</strong> 平面上的点&nbsp;<code>(0,0)</code>&nbsp;开始，先向北移动 <code>distance[0]</code> 米，然后向西移动 <code>distance[1]</code> 米，向南移动 <code>distance[2]</code> 米，向东移动 <code>distance[3]</code> 米，持续移动。也就是说，每次移动后你的方位会发生逆时针变化。</p>

<p>判断你所经过的路径是否相交。如果相交，返回 <code>true</code> ；否则，返回 <code>false</code> 。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/selfcross1-plane.jpg" style="width: 400px; height: 435px;" />
<pre>
<strong>输入：</strong>distance = [2,1,1,2]
<strong>输出：</strong>true
</pre>

<p><strong>示例 2：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/selfcross2-plane.jpg" style="width: 400px; height: 435px;" />
<pre>
<strong>输入：</strong>distance = [1,2,3,4]
<strong>输出：</strong>false
</pre>

<p><strong>示例 3：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/selfcross3-plane.jpg" style="width: 400px; height: 435px;" />
<pre>
<strong>输入：</strong>distance = [1,1,1,1]
<strong>输出：</strong>true</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;=&nbsp;distance.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;=&nbsp;distance[i] &lt;= 10<sup>5</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/self-crossing/
// discuss: https://leetcode.com/problems/self-crossing/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000335() {
    }
}