/**
 * [P002471] Minimum Number of Operations to Sort a Binary Tree by Level
 *
 * <p>给你一个 <strong>值互不相同</strong> 的二叉树的根节点 <code>root</code> 。</p>

<p>在一步操作中，你可以选择 <strong>同一层</strong> 上任意两个节点，交换这两个节点的值。</p>

<p>返回每一层按 <strong>严格递增顺序</strong> 排序所需的最少操作数目。</p>

<p>节点的 <strong>层数</strong> 是该节点和根节点之间的路径的边数。</p>

<p>&nbsp;</p>

<p><strong>示例 1 ：</strong></p>
<img src="https://assets.leetcode.com/uploads/2022/09/18/image-20220918174006-2.png" style="width: 500px; height: 324px;">
<pre><strong>输入：</strong>root = [1,4,3,7,6,8,5,null,null,null,null,9,null,10]
<strong>输出：</strong>3
<strong>解释：</strong>
- 交换 4 和 3 。第 2 层变为 [3,4] 。
- 交换 7 和 5 。第 3 层变为 [5,6,8,7] 。
- 交换 8 和 7 。第 3 层变为 [5,6,7,8] 。
共计用了 3 步操作，所以返回 3 。
可以证明 3 是需要的最少操作数目。
</pre>

<p><strong>示例 2 ：</strong></p>
<img src="https://assets.leetcode.com/uploads/2022/09/18/image-20220918174026-3.png" style="width: 400px; height: 303px;">
<pre><strong>输入：</strong>root = [1,3,2,7,6,5,4]
<strong>输出：</strong>3
<strong>解释：
</strong>- 交换 3 和 2 。第 2 层变为 [2,3] 。 
- 交换 7 和 4 。第 3 层变为 [4,6,5,7] 。 
- 交换 6 和 5 。第 3 层变为 [4,5,6,7] 。
共计用了 3 步操作，所以返回 3 。 
可以证明 3 是需要的最少操作数目。
</pre>

<p><strong>示例 3 ：</strong></p>
<img src="https://assets.leetcode.com/uploads/2022/09/18/image-20220918174052-4.png" style="width: 400px; height: 274px;">
<pre><strong>输入：</strong>root = [1,2,3,4,5,6]
<strong>输出：</strong>0
<strong>解释：</strong>每一层已经按递增顺序排序，所以返回 0 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li>树中节点的数目在范围 <code>[1, 10<sup>5</sup>]</code> 。</li>
	<li><code>1 &lt;= Node.val &lt;= 10<sup>5</sup></code></li>
	<li>树中的所有值 <strong>互不相同</strong> 。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/
// discuss: https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/discuss/

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
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002471() {
    }
}