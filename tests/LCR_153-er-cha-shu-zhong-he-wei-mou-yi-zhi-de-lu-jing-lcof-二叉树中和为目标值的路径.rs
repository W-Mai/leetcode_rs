/**
 * [LCR 153] 二叉树中和为目标值的路径
 *
 * <p>给你二叉树的根节点 <code>root</code> 和一个整数目标和 <code>targetSum</code> ，找出所有 <strong>从根节点到叶子节点</strong> 路径总和等于给定目标和的路径。</p>

<p><strong>叶子节点</strong> 是指没有子节点的节点。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/pathsumii1.jpg" /></p>

<pre>
<strong>输入：</strong>root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
<strong>输出：</strong>[[5,4,11,2],[5,8,4,5]]
</pre>

<p><strong>示例 2：</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/pathsum2.jpg" /></p>

<pre>
<strong>输入：</strong>root = [1,2,3], targetSum = 5
<strong>输出：</strong>[]
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>root = [1,2], targetSum = 0
<strong>输出：</strong>[]
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li>树中节点总数在范围 <code>[0, 5000]</code> 内</li>
	<li><code>-1000 &lt;= Node.val &lt;= 1000</code></li>
	<li><code>-1000 &lt;= targetSum &lt;= 1000</code></li>
</ul>

<p>注意：本题与主站 113&nbsp;题相同：<a href="https://leetcode-cn.com/problems/path-sum-ii/">https://leetcode-cn.com/problems/path-sum-ii/</a></p>

<p>&nbsp;</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/er-cha-shu-zhong-he-wei-mou-yi-zhi-de-lu-jing-lcof/
// discuss: https://leetcode.com/problems/er-cha-shu-zhong-he-wei-mou-yi-zhi-de-lu-jing-lcof/discuss/

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
    pub fn path_target(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Vec<i32>> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCR 153() {
    }
}