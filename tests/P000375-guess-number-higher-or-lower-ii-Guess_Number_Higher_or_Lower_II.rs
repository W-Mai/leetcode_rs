/**
 * [P000375] Guess Number Higher or Lower II
 *
 * <p>我们正在玩一个猜数游戏，游戏规则如下：</p>

<ol>
	<li>我从&nbsp;<code>1</code><strong>&nbsp;</strong>到 <code>n</code> 之间选择一个数字。</li>
	<li>你来猜我选了哪个数字。</li>
	<li>如果你猜到正确的数字，就会 <strong>赢得游戏</strong> 。</li>
	<li>如果你猜错了，那么我会告诉你，我选的数字比你的 <strong>更大或者更小</strong> ，并且你需要继续猜数。</li>
	<li>每当你猜了数字 <code>x</code> 并且猜错了的时候，你需要支付金额为 <code>x</code> 的现金。如果你花光了钱，就会<strong> 输掉游戏</strong> 。</li>
</ol>

<p>给你一个特定的数字 <code>n</code> ，返回能够 <strong>确保你获胜</strong> 的最小现金数，<strong>不管我选择那个数字</strong> 。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/10/graph.png" style="width: 505px; height: 388px;" />
<pre>
<strong>输入：</strong>n = 10
<strong>输出：</strong>16
<strong>解释：</strong>制胜策略如下：
- 数字范围是 [1,10] 。你先猜测数字为 7 。
&nbsp;   - 如果这是我选中的数字，你的总费用为 $0 。否则，你需要支付 $7 。
&nbsp;   - 如果我的数字更大，则下一步需要猜测的数字范围是 [8,10] 。你可以猜测数字为 9 。
&nbsp;       - 如果这是我选中的数字，你的总费用为 $7 。否则，你需要支付 $9 。
&nbsp;       - 如果我的数字更大，那么这个数字一定是 10 。你猜测数字为 10 并赢得游戏，总费用为 $7 + $9 = $16 。
&nbsp;       - 如果我的数字更小，那么这个数字一定是 8 。你猜测数字为 8 并赢得游戏，总费用为 $7 + $9 = $16 。
&nbsp;   - 如果我的数字更小，则下一步需要猜测的数字范围是 [1,6] 。你可以猜测数字为 3 。
&nbsp;       - 如果这是我选中的数字，你的总费用为 $7 。否则，你需要支付 $3 。
&nbsp;       - 如果我的数字更大，则下一步需要猜测的数字范围是 [4,6] 。你可以猜测数字为 5 。
&nbsp;           - 如果这是我选中的数字，你的总费用为 $7 + $3 = $10 。否则，你需要支付 $5 。
&nbsp;           - 如果我的数字更大，那么这个数字一定是 6 。你猜测数字为 6 并赢得游戏，总费用为 $7 + $3 + $5 = $15 。
&nbsp;           - 如果我的数字更小，那么这个数字一定是 4 。你猜测数字为 4 并赢得游戏，总费用为 $7 + $3 + $5 = $15 。
&nbsp;       - 如果我的数字更小，则下一步需要猜测的数字范围是 [1,2] 。你可以猜测数字为 1 。
&nbsp;           - 如果这是我选中的数字，你的总费用为 $7 + $3 = $10 。否则，你需要支付 $1 。
&nbsp;           - 如果我的数字更大，那么这个数字一定是 2 。你猜测数字为 2 并赢得游戏，总费用为 $7 + $3 + $1 = $11 。
在最糟糕的情况下，你需要支付 $16 。因此，你只需要 $16 就可以确保自己赢得游戏。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>n = 1
<strong>输出：</strong>0
<strong>解释：</strong>只有一个可能的数字，所以你可以直接猜 1 并赢得游戏，无需支付任何费用。
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>n = 2
<strong>输出：</strong>1
<strong>解释：</strong>有两个可能的数字 1 和 2 。
- 你可以先猜 1 。
&nbsp;   - 如果这是我选中的数字，你的总费用为 $0 。否则，你需要支付 $1 。
&nbsp;   - 如果我的数字更大，那么这个数字一定是 2 。你猜测数字为 2 并赢得游戏，总费用为 $1 。
最糟糕的情况下，你需要支付 $1 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 200</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/guess-number-higher-or-lower-ii/
// discuss: https://leetcode.com/problems/guess-number-higher-or-lower-ii/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000375() {
    }
}