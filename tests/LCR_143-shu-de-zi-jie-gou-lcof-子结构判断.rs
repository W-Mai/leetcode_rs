/**
 * [LCR 143] 子结构判断
 *
 * <p>给定两棵二叉树 <code>tree1</code> 和 <code>tree2</code>，判断 <code>tree2</code> 是否以 <code>tree1</code> 的某个节点为根的子树具有 <strong>相同的结构和节点值</strong> 。<br />
注意，<strong>空树&nbsp;</strong>不会是以 <code>tree1</code> 的某个节点为根的子树具有 <strong>相同的结构和节点值</strong> 。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<p>&nbsp;</p>

<p><img alt="" src="https://pic.leetcode.cn/1694684670-vwyIgY-two_tree.png" /></p>

<p>&nbsp;</p>

<pre>
<strong>输入：</strong>tree1 = [1,7,5], tree2 = [6,1]
<strong>输出：</strong>false
<strong>解释：</strong>tree2 与 tree1 的一个子树没有相同的结构和节点值。
</pre>

<p><strong>示例 2：</strong></p>

<p><img alt="" src="https://pic.leetcode.cn/1694685602-myWXCv-two_tree_2.png" /></p>

<pre>
<strong>输入：</strong>tree1 = [3,6,7,1,8], tree2 = [6,1]
<strong>输出：</strong>true
<strong>解释：</strong>tree2 与 tree1 的一个子树拥有相同的结构和节点值。即 6 - &gt; 1。</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<p><code>0 &lt;= 节点个数 &lt;= 10000</code></p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/shu-de-zi-jie-gou-lcof/
// discuss: https://leetcode.com/problems/shu-de-zi-jie-gou-lcof/discuss/

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
    pub fn is_sub_structure(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCR 143() {
    }
}