/**
 * [P000958] Check Completeness of a Binary Tree
 *
 * <p>给你一棵二叉树的根节点<meta charset="UTF-8" />&nbsp;<code>root</code> ，请你判断这棵树是否是一棵 <strong>完全二叉树</strong>&nbsp;。</p>

<p>在一棵 <strong><a href="https://baike.baidu.com/item/完全二叉树/7773232?fr=aladdin" target="_blank">完全二叉树</a></strong> 中，除了最后一层外，所有层都被完全填满，并且最后一层中的所有节点都尽可能靠左。最后一层（第 <code>h</code> 层）中可以包含<meta charset="UTF-8" />&nbsp;<code>1</code>&nbsp;到<meta charset="UTF-8" />&nbsp;<code>2<sup>h</sup></code> 个节点。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<p><img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/15/complete-binary-tree-1.png" /></p>

<pre>
<strong>输入：</strong>root = [1,2,3,4,5,6]
<strong>输出：</strong>true
<strong>解释：</strong>最后一层前的每一层都是满的（即，节点值为 {1} 和 {2,3} 的两层），且最后一层中的所有节点（{4,5,6}）尽可能靠左。
</pre>

<p><strong>示例 2：</strong></p>

<p><strong><img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/15/complete-binary-tree-2.png" /></strong></p>

<pre>
<strong>输入：</strong>root = [1,2,3,4,5,null,7]
<strong>输出：</strong>false
<strong>解释：</strong>值为 7 的节点不满足条件「节点尽可能靠左」。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li>树中节点数目在范围 <code>[1, 100]</code> 内</li>
	<li><code>1 &lt;= Node.val &lt;= 1000</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/check-completeness-of-a-binary-tree/
// discuss: https://leetcode.com/problems/check-completeness-of-a-binary-tree/discuss/

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
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000958() {
    }
}