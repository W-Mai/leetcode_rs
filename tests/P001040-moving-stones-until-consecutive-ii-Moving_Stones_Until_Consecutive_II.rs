/**
 * [P001040] Moving Stones Until Consecutive II
 *
 * <p>在一个长度 <strong>无限 </strong>的数轴上，第 <code>i</code> 颗石子的位置为 <code>stones[i]</code>。如果一颗石子的位置最小/最大，那么该石子被称作 <strong>端点石子 </strong>。</p>

<p>每个回合，你可以将一颗端点石子拿起并移动到一个未占用的位置，使得该石子不再是一颗端点石子。</p>

<p>值得注意的是，如果石子像 <code>stones = [1,2,5]</code> 这样，你将 <strong>无法 </strong>移动位于位置 5 的端点石子，因为无论将它移动到任何位置（例如 0 或 3），该石子都仍然会是端点石子。</p>

<p>当你无法进行任何移动时，即，这些石子的位置连续时，游戏结束。</p>

<p>要使游戏结束，你可以执行的最小和最大移动次数分别是多少？ 以长度为 2 的数组形式返回答案：<code>answer = [minimum_moves, maximum_moves]</code> 。</p>

<p> </p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>[7,4,9]
<strong>输出：</strong>[1,2]
<strong>解释：</strong>
我们可以移动一次，4 -> 8，游戏结束。
或者，我们可以移动两次 9 -> 5，4 -> 6，游戏结束。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>[6,5,4,3,10]
<strong>输出：</strong>[2,3]
<strong>解释：</strong>
我们可以移动 3 -> 8，接着是 10 -> 7，游戏结束。
或者，我们可以移动 3 -> 7, 4 -> 8, 5 -> 9，游戏结束。
注意，我们无法进行 10 -> 2 这样的移动来结束游戏，因为这是不合要求的移动。
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>[100,101,104,102,103]
<strong>输出：</strong>[0,0]</pre>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>3 <= stones.length <= 10^4</code></li>
	<li><code>1 <= stones[i] <= 10^9</code></li>
	<li><code>stones[i]</code> 的值各不相同。</li>
</ul>

<p> </p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/moving-stones-until-consecutive-ii/
// discuss: https://leetcode.com/problems/moving-stones-until-consecutive-ii/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001040() {
    }
}