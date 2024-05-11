/**
 * [P002718] Sum of Matrix After Queries
 *
 * <p>给你一个整数&nbsp;<code>n</code>&nbsp;和一个下标从 <strong>0</strong>&nbsp;开始的 <strong>二维数组</strong>&nbsp;<code>queries</code>&nbsp;，其中&nbsp;<code>queries[i] = [type<sub>i</sub>, index<sub>i</sub>, val<sub>i</sub>]</code>&nbsp;。</p>

<p>一开始，给你一个下标从 <strong>0</strong>&nbsp;开始的&nbsp;<code>n x n</code>&nbsp;矩阵，所有元素均为 <code>0</code>&nbsp;。每一个查询，你需要执行以下操作之一：</p>

<ul>
	<li>如果&nbsp;<code>type<sub>i</sub> == 0</code>&nbsp;，将第&nbsp;<code>index<sub>i</sub></code>&nbsp;行的元素全部修改为&nbsp;<code>val<sub>i</sub></code>&nbsp;，覆盖任何之前的值。</li>
	<li>如果&nbsp;<code>type<sub>i</sub> == 1</code>&nbsp;，将第&nbsp;<code>index<sub>i</sub></code>&nbsp;列的元素全部修改为 <code>val<sub>i</sub></code>&nbsp;，覆盖任何之前的值。</li>
</ul>

<p>请你执行完所有查询以后，返回矩阵中所有整数的和。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2023/05/11/exm1.png" style="width: 681px; height: 161px;"></p>

<pre><b>输入：</b>n = 3, queries = [[0,0,1],[1,2,2],[0,2,3],[1,0,4]]
<b>输出：</b>23
<b>解释：</b>上图展示了每个查询以后矩阵的值。所有操作执行完以后，矩阵元素之和为 23 。
</pre>

<p><strong>示例 2：</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2023/05/11/exm2.png" style="width: 681px; height: 331px;"></p>

<pre><b>输入：</b>n = 3, queries = [[0,0,4],[0,1,2],[1,0,1],[0,2,3],[1,2,1]]
<b>输出：</b>17
<b>解释：</b>上图展示了每一个查询操作之后的矩阵。所有操作执行完以后，矩阵元素之和为 17 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>queries[i].length == 3</code></li>
	<li><code>0 &lt;= type<sub>i</sub> &lt;= 1</code></li>
	<li><code>0 &lt;= index<sub>i</sub>&nbsp;&lt; n</code></li>
	<li><code>0 &lt;= val<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/sum-of-matrix-after-queries/
// discuss: https://leetcode.com/problems/sum-of-matrix-after-queries/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002718() {
    }
}