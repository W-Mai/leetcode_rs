/**
 * [P000230] Kth Smallest Element in a BST
 *
 * <p>给定一个二叉搜索树的根节点 <code>root</code> ，和一个整数 <code>k</code> ，请你设计一个算法查找其中第 <code>k</code><strong> </strong>个最小元素（从 1 开始计数）。</p>

<p> </p>

<p><strong>示例 1：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/01/28/kthtree1.jpg" style="width: 212px; height: 301px;" />
<pre>
<strong>输入：</strong>root = [3,1,4,null,2], k = 1
<strong>输出：</strong>1
</pre>

<p><strong>示例 2：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/01/28/kthtree2.jpg" style="width: 382px; height: 302px;" />
<pre>
<strong>输入：</strong>root = [5,3,6,2,4,null,null,1], k = 3
<strong>输出：</strong>3
</pre>

<p> </p>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li>树中的节点数为 <code>n</code> 。</li>
	<li><code>1 <= k <= n <= 10<sup>4</sup></code></li>
	<li><code>0 <= Node.val <= 10<sup>4</sup></code></li>
</ul>

<p> </p>

<p><strong>进阶：</strong>如果二叉搜索树经常被修改（插入/删除操作）并且你需要频繁地查找第 <code>k</code> 小的值，你将如何优化算法？</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/kth-smallest-element-in-a-bst/
// discuss: https://leetcode.com/problems/kth-smallest-element-in-a-bst/discuss/

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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000230() {
    }
}