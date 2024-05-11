/**
 * [P002551] Put Marbles in Bags
 *
 * <p>你有&nbsp;<code>k</code>&nbsp;个背包。给你一个下标从 <strong>0</strong>&nbsp;开始的整数数组&nbsp;<code>weights</code>&nbsp;，其中&nbsp;<code>weights[i]</code>&nbsp;是第&nbsp;<code>i</code>&nbsp;个珠子的重量。同时给你整数 <code>k</code>&nbsp;。</p>

<p>请你按照如下规则将所有的珠子放进&nbsp;<code>k</code>&nbsp;个背包。</p>

<ul>
	<li>没有背包是空的。</li>
	<li>如果第&nbsp;<code>i</code>&nbsp;个珠子和第&nbsp;<code>j</code>&nbsp;个珠子在同一个背包里，那么下标在&nbsp;<code>i</code>&nbsp;到&nbsp;<code>j</code>&nbsp;之间的所有珠子都必须在这同一个背包中。</li>
	<li>如果一个背包有下标从&nbsp;<code>i</code>&nbsp;到&nbsp;<code>j</code>&nbsp;的所有珠子，那么这个背包的价格是&nbsp;<code>weights[i] + weights[j]</code>&nbsp;。</li>
</ul>

<p>一个珠子分配方案的 <strong>分数</strong>&nbsp;是所有 <code>k</code>&nbsp;个背包的价格之和。</p>

<p>请你返回所有分配方案中，<strong>最大分数</strong>&nbsp;与 <strong>最小分数</strong>&nbsp;的 <strong>差值</strong>&nbsp;为多少。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><b>输入：</b>weights = [1,3,5,1], k = 2
<b>输出：</b>4
<b>解释：</b>
分配方案 [1],[3,5,1] 得到最小得分 (1+1) + (3+1) = 6 。
分配方案 [1,3],[5,1] 得到最大得分 (1+3) + (5+1) = 10 。
所以差值为 10 - 6 = 4 。
</pre>

<p><strong>示例 2：</strong></p>

<pre><b>输入：</b>weights = [1, 3], k = 2
<b>输出：</b>0
<b>解释：</b>唯一的分配方案为 [1],[3] 。
最大最小得分相等，所以返回 0 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= weights.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= weights[i] &lt;= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/put-marbles-in-bags/
// discuss: https://leetcode.com/problems/put-marbles-in-bags/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002551() {
    }
}