/**
 * [P000106] Construct Binary Tree from Inorder and Postorder Traversal
 *
 * <p>给定两个整数数组 <code>inorder</code> 和 <code>postorder</code> ，其中 <code>inorder</code> 是二叉树的中序遍历， <code>postorder</code> 是同一棵树的后序遍历，请你构造并返回这颗&nbsp;<em>二叉树</em>&nbsp;。</p>

<p>&nbsp;</p>

<p><strong>示例 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree.jpg" />
<pre>
<b>输入：</b>inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
<b>输出：</b>[3,9,20,null,null,15,7]
</pre>

<p><strong>示例 2:</strong></p>

<pre>
<b>输入：</b>inorder = [-1], postorder = [-1]
<b>输出：</b>[-1]
</pre>

<p>&nbsp;</p>

<p><strong>提示:</strong></p>

<ul>
	<li><code>1 &lt;= inorder.length &lt;= 3000</code></li>
	<li><code>postorder.length == inorder.length</code></li>
	<li><code>-3000 &lt;= inorder[i], postorder[i] &lt;= 3000</code></li>
	<li><code>inorder</code>&nbsp;和&nbsp;<code>postorder</code>&nbsp;都由 <strong>不同</strong> 的值组成</li>
	<li><code>postorder</code>&nbsp;中每一个值都在&nbsp;<code>inorder</code>&nbsp;中</li>
	<li><code>inorder</code>&nbsp;<strong>保证</strong>是树的中序遍历</li>
	<li><code>postorder</code>&nbsp;<strong>保证</strong>是树的后序遍历</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
// discuss: https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/discuss/

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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000106() {
    }
}