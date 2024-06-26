/**
 * [LCR 124] 推理二叉树
 *
 * <p>某二叉树的先序遍历结果记录于整数数组 <code>preorder</code>，它的中序遍历结果记录于整数数组 <code>inorder</code>。请根据 <code>preorder</code> 和 <code>inorder</code> 的提示构造出这棵二叉树并返回其根节点。</p>

<p>&nbsp;</p>

<p>注意：<code>preorder</code> 和 <code>inorder</code> 中均不含重复数字。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree.jpg" /></p>

<pre>
<strong>输入: </strong>preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]

<strong>输出: </strong>[3,9,20,null,null,15,7]
</pre>

<p>&nbsp;</p>

<p><strong>示例 2:</strong></p>

<pre>
<strong>输入: </strong>preorder = [-1], inorder = [-1]

<strong>输出:</strong> [-1]
</pre>

<p>&nbsp;</p>

<p><strong>提示:</strong></p>

<ul>
	<li><code>1 &lt;= preorder.length &lt;= 3000</code></li>
	<li><code>inorder.length == preorder.length</code></li>
	<li><code>-3000 &lt;= preorder[i], inorder[i] &lt;= 3000</code></li>
	<li><code>inorder</code> 均出现在 <code>preorder</code></li>
	<li><code>preorder</code> 保证 为二叉树的前序遍历序列</li>
	<li><code>inorder</code> 保证 为二叉树的中序遍历序列</li>
</ul>

<p>&nbsp;</p>

<p>注意：本题与主站 105 题重复：<a href="https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/" rel="noopener noreferrer" target="_blank">https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/</a></p>

<p>&nbsp;</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/zhong-jian-er-cha-shu-lcof/
// discuss: https://leetcode.com/problems/zhong-jian-er-cha-shu-lcof/discuss/

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
    pub fn deduce_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCR 124() {
    }
}