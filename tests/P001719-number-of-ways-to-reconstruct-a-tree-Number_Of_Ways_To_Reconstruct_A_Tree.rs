/**
 * [P001719] Number Of Ways To Reconstruct A Tree
 *
 * <p>给你一个数组 <code>pairs</code> ，其中 <code>pairs[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> ，并且满足：</p>

<ul>
	<li><code>pairs</code> 中没有重复元素</li>
	<li><code>x<sub>i</sub> < y<sub>i</sub></code></li>
</ul>

<p>令 <code>ways</code> 为满足下面条件的有根树的方案数：</p>

<ul>
	<li>树所包含的所有节点值都在 <code>pairs</code> 中。</li>
	<li>一个数对 <code>[x<sub>i</sub>, y<sub>i</sub>]</code> 出现在 <code>pairs</code> 中 <strong>当且仅当</strong><strong> </strong><code>x<sub>i</sub></code> 是 <code>y<sub>i</sub></code> 的祖先或者 <code>y<sub>i</sub></code> 是 <code>x<sub>i</sub></code><sub> </sub>的祖先。</li>
	<li><strong>注意：</strong>构造出来的树不一定是二叉树。</li>
</ul>

<p>两棵树被视为不同的方案当存在至少一个节点在两棵树中有不同的父节点。</p>

<p>请你返回：</p>

<ul>
	<li>如果 <code>ways == 0</code> ，返回 <code>0</code> 。</li>
	<li>如果 <code>ways == 1</code> ，返回 <code>1</code> 。</li>
	<li>如果 <code>ways > 1</code> ，返回 <code>2</code> 。</li>
</ul>

<p>一棵 <strong>有根树</strong> 指的是只有一个根节点的树，所有边都是从根往外的方向。</p>

<p>我们称从根到一个节点路径上的任意一个节点（除去节点本身）都是该节点的 <strong>祖先</strong> 。根节点没有祖先。</p>

<p> </p>

<p><strong>示例 1：</strong></p>
<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/01/09/trees2.png" style="width: 208px; height: 221px;" />
<pre>
<b>输入：</b>pairs = [[1,2],[2,3]]
<b>输出：</b>1
<b>解释：</b>如上图所示，有且只有一个符合规定的有根树。
</pre>

<p><strong>示例 2：</strong></p>
<img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/01/09/tree.png" style="width: 234px; height: 241px;" />
<pre>
<b>输入：</b>pairs = [[1,2],[2,3],[1,3]]
<b>输出：</b>2
<b>解释：</b>有多个符合规定的有根树，其中三个如上图所示。
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<b>输入：</b>pairs = [[1,2],[2,3],[2,4],[1,5]]
<b>输出：</b>0
<b>解释：</b>没有符合规定的有根树。</pre>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 <= pairs.length <= 10<sup>5</sup></code></li>
	<li><code>1 <= x<sub>i </sub>< y<sub>i</sub> <= 500</code></li>
	<li><code>pairs</code> 中的元素互不相同。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/number-of-ways-to-reconstruct-a-tree/
// discuss: https://leetcode.com/problems/number-of-ways-to-reconstruct-a-tree/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001719() {
    }
}