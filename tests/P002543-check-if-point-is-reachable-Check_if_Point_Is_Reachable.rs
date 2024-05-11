/**
 * [P002543] Check if Point Is Reachable
 *
 * <p>给你一个无穷大的网格图。一开始你在&nbsp;<code>(1, 1)</code>&nbsp;，你需要通过有限步移动到达点&nbsp;<code>(targetX, targetY)</code>&nbsp;。</p>

<p><b>每一步</b>&nbsp;，你可以从点&nbsp;<code>(x, y)</code>&nbsp;移动到以下点之一：</p>

<ul>
	<li><code>(x, y - x)</code></li>
	<li><code>(x - y, y)</code></li>
	<li><code>(2 * x, y)</code></li>
	<li><code>(x, 2 * y)</code></li>
</ul>

<p>给你两个整数&nbsp;<code>targetX</code> 和&nbsp;<code>targetY</code>&nbsp;，分别表示你最后需要到达点的 X 和 Y 坐标。如果你可以从&nbsp;<code>(1, 1)</code>&nbsp;出发到达这个点，请你返回<code>true</code> ，否则返回<em>&nbsp;</em><code>false</code><em>&nbsp;</em>。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><b>输入：</b>targetX = 6, targetY = 9
<b>输出：</b>false
<b>解释：</b>没法从 (1,1) 出发到达 (6,9) ，所以返回 false 。
</pre>

<p><strong>示例 2：</strong></p>

<pre><b>输入：</b>targetX = 4, targetY = 7
<b>输出：</b>true
<b>解释：</b>你可以按照以下路径到达：(1,1) -&gt; (1,2) -&gt; (1,4) -&gt; (1,8) -&gt; (1,7) -&gt; (2,7) -&gt; (4,7) 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= targetX, targetY&nbsp;&lt;= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/check-if-point-is-reachable/
// discuss: https://leetcode.com/problems/check-if-point-is-reachable/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn is_reachable(target_x: i32, target_y: i32) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002543() {
    }
}