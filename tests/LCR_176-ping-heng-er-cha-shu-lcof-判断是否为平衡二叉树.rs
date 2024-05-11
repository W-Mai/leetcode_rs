/**
 * [LCR 176] 判断是否为平衡二叉树
 *
 * <p>输入一棵二叉树的根节点，判断该树是不是平衡二叉树。如果某二叉树中任意节点的左右子树的深度相差不超过1，那么它就是一棵平衡二叉树。</p>

<p>&nbsp;</p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入：</strong>root = [3,9,20,null,null,15,7]
<strong>输出：</strong>true 
<strong>解释：</strong>如下图
</pre>

<p><img alt="" src="https://pic.leetcode.cn/1695102431-vbmWJn-image.png" style="height: 281px; width: 500px;" /><br />
<br />
<strong>示例 2:</strong></p>

<pre>
输入：root = [1,2,2,3,3,null,null,4,4]
输出：false
解释：如下图
</pre>
<img alt="" src="https://pic.leetcode.cn/1695102434-WlaxCo-image.png" style="height: 281px; width: 500px;" />
<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>0 &lt;= 树的结点个数 &lt;= 10000</code></li>
</ul>

<p>注意：本题与主站 110&nbsp;题相同：<a href="https://leetcode-cn.com/problems/balanced-binary-tree/">https://leetcode-cn.com/problems/balanced-binary-tree/</a></p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/ping-heng-er-cha-shu-lcof/
// discuss: https://leetcode.com/problems/ping-heng-er-cha-shu-lcof/discuss/

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCR 176() {
    }
}