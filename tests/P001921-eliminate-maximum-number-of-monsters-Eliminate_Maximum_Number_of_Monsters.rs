/**
 * [P001921] Eliminate Maximum Number of Monsters
 *
 * <p>你正在玩一款电子游戏，在游戏中你需要保护城市免受怪物侵袭。给定一个 <strong>下标从 0 开始</strong> 且大小为 <code>n</code> 的整数数组 <code>dist</code> ，其中 <code>dist[i]</code> 是第 <code>i</code> 个怪物与城市的 <strong>初始距离</strong>（单位：千米）。</p>

<p>怪物以 <strong>恒定</strong> 的速度走向城市。每个怪物的速度都以一个长度为 <code>n</code> 的整数数组 <code>speed</code> 表示，其中 <code>speed[i]</code> 是第 <code>i</code> 个怪物的速度（单位：千米/分）。</p>

<p>你有一种武器，一旦充满电，就可以消灭 <strong>一个</strong> 怪物。但是，武器需要 <strong>一分钟</strong> 才能充电。武器在游戏开始时是充满电的状态，怪物从 <strong>第 0 分钟</strong> 时开始移动。</p>

<p>一旦任一怪物到达城市，你就输掉了这场游戏。如果某个怪物 <strong>恰好</strong>&nbsp;在某一分钟开始时到达城市（距离表示为0），这也会被视为<strong> 输掉</strong>&nbsp;游戏，在你可以使用武器之前，游戏就会结束。</p>

<p>返回在你输掉游戏前可以消灭的怪物的 <strong>最大</strong> 数量。如果你可以在所有怪物到达城市前将它们全部消灭，返回&nbsp; <code>n</code> 。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>dist = [1,3,4], speed = [1,1,1]
<strong>输出：</strong>3
<strong>解释：</strong>
第 0 分钟开始时，怪物的距离是 [1,3,4]，你消灭了第一个怪物。
第 1 分钟开始时，怪物的距离是 [X,2,3]，你消灭了第二个怪物。
第 3 分钟开始时，怪物的距离是 [X,X,2]，你消灭了第三个怪物。
所有 3 个怪物都可以被消灭。</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>dist = [1,1,2,3], speed = [1,1,1,1]
<strong>输出：</strong>1
<strong>解释：</strong>
第 0 分钟开始时，怪物的距离是 [1,1,2,3]，你消灭了第一个怪物。
第 1 分钟开始时，怪物的距离是 [X,0,1,2]，所以你输掉了游戏。
你只能消灭 1 个怪物。
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>dist = [3,2,4], speed = [5,3,2]
<strong>输出：</strong>1
<strong>解释：</strong>
第 0 分钟开始时，怪物的距离是 [3,2,4]，你消灭了第一个怪物。
第 1 分钟开始时，怪物的距离是 [X,0,2]，你输掉了游戏。 
你只能消灭 1 个怪物。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>n == dist.length == speed.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= dist[i], speed[i] &lt;= 10<sup>5</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/eliminate-maximum-number-of-monsters/
// discuss: https://leetcode.com/problems/eliminate-maximum-number-of-monsters/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001921() {
    }
}