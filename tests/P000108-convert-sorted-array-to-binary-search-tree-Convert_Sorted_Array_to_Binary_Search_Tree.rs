/**
 * [P000108] Convert Sorted Array to Binary Search Tree
 *
 * <p>给你一个整数数组 <code>nums</code> ，其中元素已经按 <strong>升序</strong> 排列，请你将其转换为一棵 <span data-keyword="height-balanced">平衡</span> 二叉搜索树。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/btree1.jpg" style="width: 302px; height: 222px;" />
<pre>
<strong>输入：</strong>nums = [-10,-3,0,5,9]
<strong>输出：</strong>[0,-3,9,-10,null,5]
<strong>解释：</strong>[0,-10,5,null,-3,null,9] 也将被视为正确答案：
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/btree2.jpg" style="width: 302px; height: 222px;" />
</pre>

<p><strong>示例 2：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/btree.jpg" style="width: 342px; height: 142px;" />
<pre>
<strong>输入：</strong>nums = [1,3]
<strong>输出：</strong>[3,1]
<strong>解释：</strong>[1,null,3] 和 [3,1] 都是高度平衡二叉搜索树。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>4</sup></code></li>
	<li><code>-10<sup>4</sup> &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>nums</code> 按 <strong>严格递增</strong> 顺序排列</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
// discuss: https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/discuss/

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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000108() {
    }
}