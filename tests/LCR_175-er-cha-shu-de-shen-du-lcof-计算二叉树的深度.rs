/**
 * [LCR 175] 计算二叉树的深度
 *
 * <p>某公司架构以二叉树形式记录，请返回该公司的层级数。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<p><img alt="" src="https://pic.leetcode.cn/1695101942-FSrxqu-image.png" /></p>

<pre>
<strong>输入：</strong>root = [1, 2, 2, 3, null, null, 5, 4, null, null, 4]
<strong>输出: </strong>4
<strong>解释: </strong>上面示例中的二叉树的最大深度是 4，沿着路径 1 -&gt; 2 -&gt; 3 -&gt; 4 或 1 -&gt; 2 -&gt; 5 -&gt; 4 到达叶节点的最长路径上有 4 个节点。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>节点总数 &lt;= 10000</code></li>
</ul>

<p>注意：本题与主站 104&nbsp;题相同：<a href="https://leetcode-cn.com/problems/maximum-depth-of-binary-tree/">https://leetcode-cn.com/problems/maximum-depth-of-binary-tree/</a></p>

<p>&nbsp;</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/er-cha-shu-de-shen-du-lcof/
// discuss: https://leetcode.com/problems/er-cha-shu-de-shen-du-lcof/discuss/

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
    pub fn calculate_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCR 175() {
    }
}