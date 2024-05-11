/**
 * [P000948] Bag of Tokens
 *
 * <p>你的初始 <strong>能量</strong> 为 <code>power</code>，初始 <strong>分数</strong> 为&nbsp;<code>0</code>，只有一包令牌以整数数组&nbsp;<code>tokens</code>&nbsp;给出。其中 <code>tokens[i]</code> 是第 <code>i</code> 个令牌的值（下标从 0 开始）。</p>

<p>你的目标是通过有策略地使用这些令牌以&nbsp;<strong>最大化</strong>&nbsp;总 <strong>分数</strong>。在一次行动中，你可以用两种方式中的一种来使用一个&nbsp;<strong>未被使用的</strong> 令牌（但不是对同一个令牌使用两种方式）：</p>

<ul>
	<li><strong>朝上</strong>：如果你当前&nbsp;<strong>至少</strong> 有&nbsp;<code>tokens[i]</code>&nbsp;点 <strong>能量</strong> ，可以使用令牌 <code>i</code> ，失去&nbsp;<code>tokens[i]</code>&nbsp;点 <strong>能量</strong> ，并得到&nbsp;<code>1</code>&nbsp;<strong>分</strong> 。</li>
	<li><strong>朝下</strong>：如果你当前至少有&nbsp;<code>1</code>&nbsp;<strong>分 </strong>，可以使用令牌 <code>i</code> ，获得&nbsp;<code>tokens[i]</code> 点 <strong>能量</strong> ，并失去&nbsp;<code>1</code>&nbsp;<strong>分</strong> 。</li>
</ul>

<p>在使用 <strong>任意</strong> 数量的令牌后，返回我们可以得到的最大 <strong>分数</strong> 。</p>

<p>&nbsp;</p>

<ol>
</ol>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>tokens = [100], power = 50
<strong>输出：</strong>0
<strong>解释：</strong>因为你的初始分数为 <code>0，</code>无法使令牌朝下。你也不能使令牌朝上因为你的能量（<code>50</code>）比 <code>tokens[0]</code>&nbsp;少（<code>100</code>）。</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>tokens = [200,100], power = 150
<strong>输出：</strong>1
<strong>解释：</strong>使令牌 1 正面朝上，能量变为 50，分数变为 1 。
不必使用令牌 0，因为你无法使用它来提高分数。可得到的最大分数是 <code>1</code>。</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>tokens = [100,200,300,400], power = 200
<strong>输出：</strong>2
<strong>解释：</strong>按下面顺序使用令牌可以得到 2 分：
1. 令牌 0 (<code>100</code>)正面朝上，能量变为 <code>100</code> ，分数变为 <code>1</code>
2. 令牌 3 (<code>400</code>)正面朝下，能量变为 <code>500</code> ，分数变为 <code>0</code>
3. 令牌 1 (<code>200</code>)正面朝上，能量变为 <code>300</code> ，分数变为 <code>1</code>
4. 令牌 2 (<code>300</code>)正面朝上，能量变为 <code>0</code> ，分数变为 <code>2</code>

可得的最大分数是 2。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>0 &lt;= tokens.length &lt;= 1000</code></li>
	<li><code>0 &lt;= tokens[i],&nbsp;power &lt; 10<sup>4</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/bag-of-tokens/
// discuss: https://leetcode.com/problems/bag-of-tokens/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000948() {
    }
}