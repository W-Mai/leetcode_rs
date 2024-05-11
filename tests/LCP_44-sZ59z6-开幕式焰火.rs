/**
 * [LCP 44] 开幕式焰火
 *
 * 「力扣挑战赛」开幕式开始了，空中绽放了一颗二叉树形的巨型焰火。
给定一棵二叉树 `root` 代表焰火，节点值表示巨型焰火这一位置的颜色种类。请帮小扣计算巨型焰火有多少种不同的颜色。


**示例 1：**
>输入：`root = [1,3,2,1,null,2]`
>
>输出：`3`
>
>解释：焰火中有 3 个不同的颜色，值分别为 1、2、3

**示例 2：**
>输入：`root = [3,3,3]`
>
>输出：`1`
>
>解释：焰火中仅出现 1 个颜色，值为 3

**提示：**
- `1 <= 节点个数 <= 1000`
- `1 <= Node.val <= 1000`



 */
pub struct Solution {}



// problem: https://leetcode.com/problems/sZ59z6/
// discuss: https://leetcode.com/problems/sZ59z6/discuss/

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
    pub fn num_color(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCP 44() {
    }
}