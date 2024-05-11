/**
 * [P001530] Number of Good Leaf Nodes Pairs
 *
 * <p>给你二叉树的根节点 <code>root</code> 和一个整数 <code>distance</code> 。</p>

<p>如果二叉树中两个 <strong>叶</strong> 节点之间的 <strong>最短路径长度</strong> 小于或者等于 <code>distance</code> ，那它们就可以构成一组 <strong>好叶子节点对</strong> 。</p>

<p>返回树中 <strong>好叶子节点对的数量</strong> 。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<p>&nbsp;</p>

<p><img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/07/26/e1.jpg" style="height: 321px; width: 321px;"></p>

<pre><strong>输入：</strong>root = [1,2,3,null,4], distance = 3
<strong>输出：</strong>1
<strong>解释：</strong>树的叶节点是 3 和 4 ，它们之间的最短路径的长度是 3 。这是唯一的好叶子节点对。
</pre>

<p><strong>示例 2：</strong></p>

<p><img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/07/26/e2.jpg" style="height: 321px; width: 441px;"></p>

<pre><strong>输入：</strong>root = [1,2,3,4,5,6,7], distance = 3
<strong>输出：</strong>2
<strong>解释：</strong>好叶子节点对为 [4,5] 和 [6,7] ，最短路径长度都是 2 。但是叶子节点对 [4,6] 不满足要求，因为它们之间的最短路径长度为 4 。
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>root = [7,1,4,6,null,5,3,null,null,null,null,null,2], distance = 3
<strong>输出：</strong>1
<strong>解释：</strong>唯一的好叶子节点对是 [2,5] 。
</pre>

<p><strong>示例 4：</strong></p>

<pre><strong>输入：</strong>root = [100], distance = 1
<strong>输出：</strong>0
</pre>

<p><strong>示例 5：</strong></p>

<pre><strong>输入：</strong>root = [1,1,1], distance = 2
<strong>输出：</strong>1
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>tree</code> 的节点数在 <code>[1, 2^10]</code> 范围内。</li>
	<li>每个节点的值都在 <code>[1, 100]</code> 之间。</li>
	<li><code>1 &lt;= distance &lt;= 10</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/number-of-good-leaf-nodes-pairs/
// discuss: https://leetcode.com/problems/number-of-good-leaf-nodes-pairs/discuss/

// >>> SUBMISSION CODES START HERE <<<

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001530() {
    }
}