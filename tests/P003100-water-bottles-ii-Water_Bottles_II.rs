/**
 * [P003100] Water Bottles II
 *
 * <p>给你两个整数 <code>numBottles</code> 和 <code>numExchange</code> 。</p>

<p><code>numBottles</code> 代表你最初拥有的满水瓶数量。在一次操作中，你可以执行以下操作之一：</p>

<ul>
	<li>喝掉任意数量的满水瓶，使它们变成空水瓶。</li>
	<li>用 <code>numExchange</code> 个空水瓶交换一个满水瓶。然后，将 <code>numExchange</code> 的值增加 1 。</li>
</ul>

<p>注意，你不能使用相同的 <code>numExchange</code> 值交换多批空水瓶。例如，如果 <code>numBottles == 3</code> 并且 <code>numExchange == 1</code> ，则不能用 <code>3</code> 个空水瓶交换成 <code>3</code> 个满水瓶。</p>

<p>返回你 <strong>最多</strong> 可以喝到多少瓶水。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2024/01/28/exampleone1.png" style="width: 948px; height: 482px; padding: 10px; background: #fff; border-radius: .5rem;" />
<pre>
<strong>输入：</strong>numBottles = 13, numExchange = 6
<strong>输出：</strong>15
<strong>解释：</strong>上表显示了满水瓶的数量、空水瓶的数量、numExchange 的值，以及累计喝掉的水瓶数量。
</pre>

<p><strong class="example">示例 2：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2024/01/28/example231.png" style="width: 990px; height: 642px; padding: 10px; background: #fff; border-radius: .5rem;" />
<pre>
<strong>输入：</strong>numBottles = 10, numExchange = 3
<strong>输出：</strong>13
<strong>解释：</strong>上表显示了满水瓶的数量、空水瓶的数量、numExchange 的值，以及累计喝掉的水瓶数量。</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= numBottles &lt;= 100 </code></li>
	<li><code>1 &lt;= numExchange &lt;= 100</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/water-bottles-ii/
// discuss: https://leetcode.com/problems/water-bottles-ii/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P003100() {
    }
}