/**
 * [P001686] Stone Game VI
 *
 * <p>Alice 和 Bob 轮流玩一个游戏，Alice 先手。</p>

<p>一堆石子里总共有 <code>n</code> 个石子，轮到某个玩家时，他可以 <strong>移出</strong> 一个石子并得到这个石子的价值。Alice 和 Bob 对石子价值有 <strong>不一样的的评判标准</strong> 。双方都知道对方的评判标准。</p>

<p>给你两个长度为 <code>n</code> 的整数数组 <code>aliceValues</code> 和 <code>bobValues</code> 。<code>aliceValues[i]</code> 和 <code>bobValues[i]</code> 分别表示 Alice 和 Bob 认为第 <code>i</code> 个石子的价值。</p>

<p>所有石子都被取完后，得分较高的人为胜者。如果两个玩家得分相同，那么为平局。两位玩家都会采用 <b>最优策略</b> 进行游戏。</p>

<p>请你推断游戏的结果，用如下的方式表示：</p>

<ul>
	<li>如果 Alice 赢，返回 <code>1</code> 。</li>
	<li>如果 Bob 赢，返回 <code>-1</code> 。</li>
	<li>如果游戏平局，返回 <code>0</code> 。</li>
</ul>

<p> </p>

<p><strong>示例 1：</strong></p>

<pre>
<b>输入：</b>aliceValues = [1,3], bobValues = [2,1]
<b>输出：</b>1
<strong>解释：</strong>
如果 Alice 拿石子 1 （下标从 0开始），那么 Alice 可以得到 3 分。
Bob 只能选择石子 0 ，得到 2 分。
Alice 获胜。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>aliceValues = [1,2], bobValues = [3,1]
<b>输出：</b>0
<strong>解释：</strong>
Alice 拿石子 0 ， Bob 拿石子 1 ，他们得分都为 1 分。
打平。
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<b>输入：</b>aliceValues = [2,4,3], bobValues = [1,6,7]
<b>输出：</b>-1
<strong>解释：</strong>
不管 Alice 怎么操作，Bob 都可以得到比 Alice 更高的得分。
比方说，Alice 拿石子 1 ，Bob 拿石子 2 ， Alice 拿石子 0 ，Alice 会得到 6 分而 Bob 得分为 7 分。
Bob 会获胜。
</pre>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>n == aliceValues.length == bobValues.length</code></li>
	<li><code>1 <= n <= 10<sup>5</sup></code></li>
	<li><code>1 <= aliceValues[i], bobValues[i] <= 100</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/stone-game-vi/
// discuss: https://leetcode.com/problems/stone-game-vi/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001686() {
    }
}