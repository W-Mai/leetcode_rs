/**
 * [P002580] Count Ways to Group Overlapping Ranges
 *
 * <p>给你一个二维整数数组&nbsp;<code>ranges</code>&nbsp;，其中&nbsp;<code>ranges[i] = [start<sub>i</sub>, end<sub>i</sub>]</code>&nbsp;表示&nbsp;<code>start<sub>i</sub></code>&nbsp;到&nbsp;<code>end<sub>i</sub></code>&nbsp;之间（包括二者）的所有整数都包含在第&nbsp;<code>i</code>&nbsp;个区间中。</p>

<p>你需要将&nbsp;<code>ranges</code>&nbsp;分成 <strong>两个</strong>&nbsp;组（可以为空），满足：</p>

<ul>
	<li>每个区间只属于一个组。</li>
	<li>两个有 <strong>交集</strong>&nbsp;的区间必须在 <strong>同一个&nbsp;</strong>组内。</li>
</ul>

<p>如果两个区间有至少 <strong>一个</strong>&nbsp;公共整数，那么这两个区间是 <b>有交集</b>&nbsp;的。</p>

<ul>
	<li>比方说，区间&nbsp;<code>[1, 3]</code> 和&nbsp;<code>[2, 5]</code>&nbsp;有交集，因为&nbsp;<code>2</code>&nbsp;和&nbsp;<code>3</code>&nbsp;在两个区间中都被包含。</li>
</ul>

<p>请你返回将 <code>ranges</code>&nbsp;划分成两个组的 <strong>总方案数</strong>&nbsp;。由于答案可能很大，将它对&nbsp;<code>10<sup>9</sup> + 7</code>&nbsp;<strong>取余</strong>&nbsp;后返回。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><b>输入：</b>ranges = [[6,10],[5,15]]
<b>输出：</b>2
<b>解释：</b>
两个区间有交集，所以它们必须在同一个组内。
所以有两种方案：
- 将两个区间都放在第 1 个组中。
- 将两个区间都放在第 2 个组中。
</pre>

<p><strong>示例 2：</strong></p>

<pre><b>输入：</b>ranges = [[1,3],[10,20],[2,5],[4,8]]
<b>输出：</b>4
<b>解释：</b>
区间 [1,3] 和 [2,5] 有交集，所以它们必须在同一个组中。
同理，区间 [2,5] 和 [4,8] 也有交集，所以它们也必须在同一个组中。
所以总共有 4 种分组方案：
- 所有区间都在第 1 组。
- 所有区间都在第 2 组。
- 区间 [1,3] ，[2,5] 和 [4,8] 在第 1 个组中，[10,20] 在第 2 个组中。
- 区间 [1,3] ，[2,5] 和 [4,8] 在第 2 个组中，[10,20] 在第 1 个组中。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= ranges.length &lt;= 10<sup>5</sup></code></li>
	<li><code>ranges[i].length == 2</code></li>
	<li><code>0 &lt;= start<sub>i</sub> &lt;= end<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/count-ways-to-group-overlapping-ranges/
// discuss: https://leetcode.com/problems/count-ways-to-group-overlapping-ranges/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002580() {
    }
}