/**
 * [P002672] Number of Adjacent Elements With the Same Color
 *
 * <p>给你一个下标从 <strong>0</strong>&nbsp;开始、长度为 <code>n</code>&nbsp;的数组&nbsp;<code>nums</code>&nbsp;。一开始，所有元素都是 <strong>未染色</strong>&nbsp;（值为 <code>0</code>&nbsp;）的。</p>

<p>给你一个二维整数数组&nbsp;<code>queries</code>&nbsp;，其中&nbsp;<code>queries[i] = [index<sub>i</sub>, color<sub>i</sub>]</code>&nbsp;。</p>

<p>对于每个操作，你需要将数组 <code>nums</code>&nbsp;中下标为&nbsp;<code>index<sub>i</sub></code>&nbsp;的格子染色为&nbsp;<code>color<sub>i</sub></code>&nbsp;。</p>

<p>请你返回一个长度与 <code>queries</code>&nbsp;相等的数组<em>&nbsp;</em><code>answer</code><em>&nbsp;</em>，其中<em>&nbsp;</em><code>answer[i]</code>是前 <code>i</code>&nbsp;个操作&nbsp;<strong>之后</strong>&nbsp;，相邻元素颜色相同的数目。</p>

<p>更正式的，<code>answer[i]</code>&nbsp;是执行完前 <code>i</code>&nbsp;个操作后，<code>0 &lt;= j &lt; n - 1</code>&nbsp;的下标 <code>j</code>&nbsp;中，满足&nbsp;<code>nums[j] == nums[j + 1]</code> 且&nbsp;<code>nums[j] != 0</code>&nbsp;的数目。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<b>输入：</b>n = 4, queries = [[0,2],[1,2],[3,1],[1,1],[2,1]]
<b>输出：</b>[0,1,1,0,2]
<b>解释：</b>一开始数组 nums = [0,0,0,0] ，0 表示数组中还没染色的元素。
- 第 1 个操作后，nums = [2,0,0,0] 。相邻元素颜色相同的数目为 0 。
- 第 2 个操作后，nums = [2,2,0,0] 。相邻元素颜色相同的数目为 1 。
- 第 3 个操作后，nums = [2,2,0,1] 。相邻元素颜色相同的数目为 1 。
- 第 4 个操作后，nums = [2,1,0,1] 。相邻元素颜色相同的数目为 0 。
- 第 5 个操作后，nums = [2,1,1,1] 。相邻元素颜色相同的数目为 2 。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<b>输入：</b>n = 1, queries = [[0,100000]]
<b>输出：</b>[0]
<b>解释：</b>一开始数组 nums = [0] ，0 表示数组中还没染色的元素。
- 第 1 个操作后，nums = [100000] 。相邻元素颜色相同的数目为 0 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>queries[i].length&nbsp;== 2</code></li>
	<li><code>0 &lt;= index<sub>i</sub>&nbsp;&lt;= n - 1</code></li>
	<li><code>1 &lt;=&nbsp; color<sub>i</sub>&nbsp;&lt;= 10<sup>5</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/number-of-adjacent-elements-with-the-same-color/
// discuss: https://leetcode.com/problems/number-of-adjacent-elements-with-the-same-color/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002672() {
    }
}