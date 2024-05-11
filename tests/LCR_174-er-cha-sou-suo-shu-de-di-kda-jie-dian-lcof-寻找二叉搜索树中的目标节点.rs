/**
 * [LCR 174] 寻找二叉搜索树中的目标节点
 *
 * <p>某公司组织架构以二叉搜索树形式记录，节点值为处于该职位的员工编号。请返回第 <code>cnt</code> 大的员工编号。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<p><img alt="" src="https://pic.leetcode.cn/1695101634-kzHKZW-image.png" style="height: 281px; width: 500px;" /></p>

<pre>
<strong>输入：</strong>root = [7, 3, 9, 1, 5], cnt = 2
       7
      / \
     3   9
    / \
   1   5
<strong>输出：</strong>7
</pre>

<p><strong>示例 2：</strong></p>

<p><img alt="" src="https://pic.leetcode.cn/1695101636-ESZtLa-image.png" style="height: 281px; width: 500px;" /></p>

<pre>
<strong>输入:</strong> root = [10, 5, 15, 2, 7, null, 20, 1, null, 6, 8], cnt = 4
       10
      / \
     5   15
    / \    \
   2   7    20
  /   / \ 
 1   6   8
<strong>输出:</strong> 8</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li>1 ≤ cnt&nbsp;≤ 二叉搜索树元素个数</li>
</ul>

<p>&nbsp;</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/er-cha-sou-suo-shu-de-di-kda-jie-dian-lcof/
// discuss: https://leetcode.com/problems/er-cha-sou-suo-shu-de-di-kda-jie-dian-lcof/discuss/

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
    pub fn find_target_node(root: Option<Rc<RefCell<TreeNode>>>, cnt: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCR 174() {
    }
}