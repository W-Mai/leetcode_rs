/**
 * [P002581] Count Number of Possible Root Nodes
 *
 * <p>Alice 有一棵 <code>n</code> 个节点的树，节点编号为 <code>0</code> 到 <code>n - 1</code> 。树用一个长度为 <code>n - 1</code> 的二维整数数组 <code>edges</code> 表示，其中 <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> ，表示树中节点 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间有一条边。</p>

<p>Alice 想要 Bob 找到这棵树的根。她允许 Bob 对这棵树进行若干次 <strong>猜测</strong> 。每一次猜测，Bob 做如下事情：</p>

<ul>
	<li>选择两个 <strong>不相等</strong>&nbsp;的整数&nbsp;<code>u</code> 和&nbsp;<code>v</code>&nbsp;，且树中必须存在边&nbsp;<code>[u, v]</code>&nbsp;。</li>
	<li>Bob 猜测树中&nbsp;<code>u</code>&nbsp;是&nbsp;<code>v</code>&nbsp;的 <strong>父节点</strong>&nbsp;。</li>
</ul>

<p>Bob 的猜测用二维整数数组&nbsp;<code>guesses</code> 表示，其中&nbsp;<code>guesses[j] = [u<sub>j</sub>, v<sub>j</sub>]</code>&nbsp;表示 Bob 猜&nbsp;<code>u<sub>j</sub></code> 是&nbsp;<code>v<sub>j</sub></code>&nbsp;的父节点。</p>

<p>Alice 非常懒，她不想逐个回答&nbsp;Bob 的猜测，只告诉 Bob 这些猜测里面 <strong>至少</strong>&nbsp;有&nbsp;<code>k</code>&nbsp;个猜测的结果为&nbsp;<code>true</code>&nbsp;。</p>

<p>给你二维整数数组 <code>edges</code>&nbsp;，Bob 的所有猜测和整数&nbsp;<code>k</code>&nbsp;，请你返回可能成为树根的&nbsp;<strong>节点数目</strong>&nbsp;。如果没有这样的树，则返回 <code>0</code>。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2022/12/19/ex-1.png" style="width: 727px; height: 250px;" /></p>

<pre>
<b>输入：</b>edges = [[0,1],[1,2],[1,3],[4,2]], guesses = [[1,3],[0,1],[1,0],[2,4]], k = 3
<b>输出：</b>3
<b>解释：</b>
根为节点 0 ，正确的猜测为 [1,3], [0,1], [2,4]
根为节点 1 ，正确的猜测为 [1,3], [1,0], [2,4]
根为节点 2 ，正确的猜测为 [1,3], [1,0], [2,4]
根为节点 3 ，正确的猜测为 [1,0], [2,4]
根为节点 4 ，正确的猜测为 [1,3], [1,0]
节点 0 ，1 或 2 为根时，可以得到 3 个正确的猜测。
</pre>

<p><strong>示例 2：</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2022/12/19/ex-2.png" style="width: 600px; height: 303px;" /></p>

<pre>
<b>输入：</b>edges = [[0,1],[1,2],[2,3],[3,4]], guesses = [[1,0],[3,4],[2,1],[3,2]], k = 1
<b>输出：</b>5
<b>解释：</b>
根为节点 0 ，正确的猜测为 [3,4]
根为节点 1 ，正确的猜测为 [1,0], [3,4]
根为节点 2 ，正确的猜测为 [1,0], [2,1], [3,4]
根为节点 3 ，正确的猜测为 [1,0], [2,1], [3,2], [3,4]
根为节点 4 ，正确的猜测为 [1,0], [2,1], [3,2]
任何节点为根，都至少有 1 个正确的猜测。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>edges.length == n - 1</code></li>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= guesses.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= a<sub>i</sub>, b<sub>i</sub>, u<sub>j</sub>, v<sub>j</sub> &lt;= n - 1</code></li>
	<li><code>a<sub>i</sub> != b<sub>i</sub></code></li>
	<li><code>u<sub>j</sub> != v<sub>j</sub></code></li>
	<li><code>edges</code>&nbsp;表示一棵有效的树。</li>
	<li><code>guesses[j]</code>&nbsp;是树中的一条边。</li>
	<li><code>guesses</code>&nbsp;是唯一的。</li>
	<li><code>0 &lt;= k &lt;= guesses.length</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/count-number-of-possible-root-nodes/
// discuss: https://leetcode.com/problems/count-number-of-possible-root-nodes/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002581() {
    }
}