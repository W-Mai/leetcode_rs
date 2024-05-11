/**
 * [P002858] Minimum Edge Reversals So Every Node Is Reachable
 *
 * <p>给你一个 <code>n</code>&nbsp;个点的 <strong>简单有向图</strong>&nbsp;（没有重复边的有向图），节点编号为 <code>0</code>&nbsp;到 <code>n - 1</code>&nbsp;。如果这些边是双向边，那么这个图形成一棵&nbsp;<strong>树</strong>&nbsp;。</p>

<p>给你一个整数&nbsp;<code>n</code>&nbsp;和一个 <strong>二维</strong>&nbsp;整数数组&nbsp;<code>edges</code>&nbsp;，其中&nbsp;<code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code>&nbsp;表示从节点&nbsp;<code>u<sub>i</sub></code>&nbsp;到节点&nbsp;<code>v<sub>i</sub></code>&nbsp;有一条&nbsp;<strong>有向边</strong>&nbsp;。</p>

<p><strong>边反转</strong>&nbsp;指的是将一条边的方向反转，也就是说一条从节点&nbsp;<code>u<sub>i</sub></code>&nbsp;到节点&nbsp;<code>v<sub>i</sub></code>&nbsp;的边会变为一条从节点&nbsp;<code>v<sub>i</sub></code>&nbsp;到节点&nbsp;<code>u<sub>i</sub></code>&nbsp;的边。</p>

<p>对于范围&nbsp;<code>[0, n - 1]</code>&nbsp;中的每一个节点 <code>i</code>&nbsp;，你的任务是分别 <strong>独立</strong> 计算 <strong>最少</strong>&nbsp;需要多少次 <strong>边反转</strong>&nbsp;，从节点 <code>i</code>&nbsp;出发经过 <strong>一系列有向边</strong>&nbsp;，可以到达所有的节点。</p>

<p>请你返回一个长度为 <code>n</code>&nbsp;的整数数组<em>&nbsp;</em><code>answer</code><em>&nbsp;</em>，其中<em>&nbsp;</em><code>answer[i]</code>表示从节点&nbsp;<code>i</code>&nbsp;出发，可以到达所有节点的&nbsp;<strong>最少边反转</strong>&nbsp;次数。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<p><img height="246" src="https://assets.leetcode.com/uploads/2023/08/26/image-20230826221104-3.png" width="312" /></p>

<pre>
<b>输入：</b>n = 4, edges = [[2,0],[2,1],[1,3]]
<b>输出：</b>[1,1,0,2]
<b>解释：</b>上图表示了与输入对应的简单有向图。
对于节点 0 ：反转 [2,0] ，从节点 0 出发可以到达所有节点。
所以 answer[0] = 1 。
对于节点 1 ：反转 [2,1] ，从节点 1 出发可以到达所有节点。
所以 answer[1] = 1 。
对于节点 2 ：不需要反转就可以从节点 2 出发到达所有节点。
所以 answer[2] = 0 。
对于节点 3 ：反转 [1,3] 和 [2,1] ，从节点 3 出发可以到达所有节点。
所以 answer[3] = 2 。
</pre>

<p><strong class="example">示例 2：</strong></p>

<p><img height="217" src="https://assets.leetcode.com/uploads/2023/08/26/image-20230826225541-2.png" width="322" /></p>

<pre>
<b>输入：</b>n = 3, edges = [[1,2],[2,0]]
<b>输出：</b>[2,0,1]
<b>解释：</b>上图表示了与输入对应的简单有向图。
对于节点 0 ：反转 [2,0] 和 [1,2] ，从节点 0 出发可以到达所有节点。
所以 answer[0] = 2 。
对于节点 1 ：不需要反转就可以从节点 2 出发到达所有节点。
所以 answer[1] = 0 。
对于节点 2 ：反转 [1,2] ，从节点 2 出发可以到达所有节点。
所以 answer[2] = 1 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i].length == 2</code></li>
	<li><code>0 &lt;= u<sub>i</sub> == edges[i][0] &lt; n</code></li>
	<li><code>0 &lt;= v<sub>i</sub> == edges[i][1] &lt; n</code></li>
	<li><code>u<sub>i</sub> != v<sub>i</sub></code></li>
	<li>输入保证如果边是双向边，可以得到一棵树。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/minimum-edge-reversals-so-every-node-is-reachable/
// discuss: https://leetcode.com/problems/minimum-edge-reversals-so-every-node-is-reachable/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn min_edge_reversals(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002858() {
    }
}