/**
 * [LCR 026] 重排链表
 *
 * <p>给定一个单链表 <code>L</code><em> </em>的头节点 <code>head</code> ，单链表 <code>L</code> 表示为：</p>

<p><code>&nbsp;L<sub>0&nbsp;</sub>&rarr; L<sub>1&nbsp;</sub>&rarr; &hellip; &rarr; L<sub>n-1&nbsp;</sub>&rarr; L<sub>n&nbsp;</sub></code><br />
请将其重新排列后变为：</p>

<p><code>L<sub>0&nbsp;</sub>&rarr;&nbsp;L<sub>n&nbsp;</sub>&rarr;&nbsp;L<sub>1&nbsp;</sub>&rarr;&nbsp;L<sub>n-1&nbsp;</sub>&rarr;&nbsp;L<sub>2&nbsp;</sub>&rarr;&nbsp;L<sub>n-2&nbsp;</sub>&rarr; &hellip;</code></p>

<p>不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。</p>

<p>&nbsp;</p>

<p><strong>示例 1:</strong></p>

<p><img alt="" src="https://pic.leetcode-cn.com/1626420311-PkUiGI-image.png" style="width: 240px; " /></p>

<pre>
<strong>输入: </strong>head = [1,2,3,4]
<strong>输出: </strong>[1,4,2,3]</pre>

<p><strong>示例 2:</strong></p>

<p><img alt="" src="https://pic.leetcode-cn.com/1626420320-YUiulT-image.png" style="width: 320px; " /></p>

<pre>
<strong>输入: </strong>head = [1,2,3,4,5]
<strong>输出: </strong>[1,5,2,4,3]</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li>链表的长度范围为 <code>[1, 5 * 10<sup>4</sup>]</code></li>
	<li><code>1 &lt;= node.val &lt;= 1000</code></li>
</ul>

<p>&nbsp;</p>

<p><meta charset="UTF-8" />注意：本题与主站 143&nbsp;题相同：<a href="https://leetcode-cn.com/problems/reorder-list/">https://leetcode-cn.com/problems/reorder-list/</a>&nbsp;</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/LGjMqU/
// discuss: https://leetcode.com/problems/LGjMqU/discuss/

// >>> SUBMISSION CODES START HERE <<<

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCR 026() {
    }
}