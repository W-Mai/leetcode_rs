/**
 * [P002731] Movement of Robots
 *
 * <p>有一些机器人分布在一条无限长的数轴上，他们初始坐标用一个下标从 <strong>0</strong>&nbsp;开始的整数数组&nbsp;<code>nums</code>&nbsp;表示。当你给机器人下达命令时，它们以每秒钟一单位的速度开始移动。</p>

<p>给你一个字符串&nbsp;<code>s</code>&nbsp;，每个字符按顺序分别表示每个机器人移动的方向。<code>'L'</code>&nbsp;表示机器人往左或者数轴的负方向移动，<code>'R'</code>&nbsp;表示机器人往右或者数轴的正方向移动。</p>

<p>当两个机器人相撞时，它们开始沿着原本相反的方向移动。</p>

<p>请你返回指令重复执行 <code>d</code>&nbsp;秒后，所有机器人之间两两距离之和。由于答案可能很大，请你将答案对&nbsp;<code>10<sup>9</sup> + 7</code>&nbsp;取余后返回。</p>

<p><b>注意：</b></p>

<ul>
	<li>对于坐标在&nbsp;<code>i</code> 和&nbsp;<code>j</code>&nbsp;的两个机器人，<code>(i,j)</code>&nbsp;和&nbsp;<code>(j,i)</code>&nbsp;视为相同的坐标对。也就是说，机器人视为无差别的。</li>
	<li>当机器人相撞时，它们 <strong>立即改变</strong>&nbsp;它们的前进方向，这个过程不消耗任何时间。</li>
	<li>
	<p>当两个机器人在同一时刻占据相同的位置时，就会相撞。</p>

	<ul>
		<li>
		<p>例如，如果一个机器人位于位置 0 并往右移动，另一个机器人位于位置 2 并往左移动，下一秒，它们都将占据位置 1，并改变方向。再下一秒钟后，第一个机器人位于位置 0 并往左移动，而另一个机器人位于位置 2 并往右移动。</p>
		</li>
		<li>
		<p>例如，如果一个机器人位于位置 0 并往右移动，另一个机器人位于位置 1 并往左移动，下一秒，第一个机器人位于位置 0 并往左行驶，而另一个机器人位于位置 1 并往右移动。</p>
		</li>
	</ul>
	</li>
</ul>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<b>输入：</b>nums = [-2,0,2], s = "RLL", d = 3
<b>输出：</b>8
<b>解释：</b>
1 秒后，机器人的位置为 [-1,-1,1] 。现在下标为 0 的机器人开始往左移动，下标为 1 的机器人开始往右移动。
2 秒后，机器人的位置为 [-2,0,0] 。现在下标为 1 的机器人开始往左移动，下标为 2 的机器人开始往右移动。
3 秒后，机器人的位置为 [-3,-1,1] 。
下标为 0 和 1 的机器人之间距离为 abs(-3 - (-1)) = 2 。
下标为 0 和 2 的机器人之间的距离为 abs(-3 - 1) = 4 。
下标为 1 和 2 的机器人之间的距离为 abs(-1 - 1) = 2 。
所有机器人对之间的总距离为 2 + 4 + 2 = 8 。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<b>输入：</b>nums = [1,0], s = "RL", d = 2
<b>输出：</b>5
<b>解释：</b>
1 秒后，机器人的位置为 [2,-1] 。
2 秒后，机器人的位置为 [3,-2] 。
两个机器人的距离为 abs(-2 - 3) = 5 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-2 * 10<sup>9</sup>&nbsp;&lt;= nums[i] &lt;= 2 * 10<sup>9</sup></code></li>
	<li><code>0 &lt;= d &lt;= 10<sup>9</sup></code></li>
	<li><code>nums.length == s.length&nbsp;</code></li>
	<li><code>s</code>&nbsp;只包含&nbsp;<code>'L'</code> 和 <code>'R'</code>&nbsp;。</li>
	<li><code>nums[i]</code>&nbsp;互不相同。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/movement-of-robots/
// discuss: https://leetcode.com/problems/movement-of-robots/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002731() {
    }
}