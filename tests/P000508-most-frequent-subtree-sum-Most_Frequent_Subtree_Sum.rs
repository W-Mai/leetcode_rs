/**
 * [P000508] Most Frequent Subtree Sum
 *
 * <p>给你一个二叉树的根结点&nbsp;<code>root</code>&nbsp;，请返回出现次数最多的子树元素和。如果有多个元素出现的次数相同，返回所有出现次数最多的子树元素和（不限顺序）。</p>

<p>一个结点的&nbsp;<strong>「子树元素和」</strong>&nbsp;定义为以该结点为根的二叉树上所有结点的元素之和（包括结点本身）。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2021/04/24/freq1-tree.jpg" /></p>

<pre>
<strong>输入:</strong> root = [5,2,-3]
<strong>输出:</strong> [2,-3,4]
</pre>

<p><strong>示例&nbsp;2：</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2021/04/24/freq2-tree.jpg" /></p>

<pre>
<strong>输入:</strong> root = [5,2,-5]
<b>输出:</b> [2]
</pre>

<p>&nbsp;</p>

<p><strong>提示:</strong></p>

<ul>
	<li>节点数在&nbsp;<code>[1, 10<sup>4</sup>]</code>&nbsp;范围内</li>
	<li><code>-10<sup>5</sup>&nbsp;&lt;= Node.val &lt;= 10<sup>5</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/most-frequent-subtree-sum/
// discuss: https://leetcode.com/problems/most-frequent-subtree-sum/discuss/

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
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000508() {
    }
}