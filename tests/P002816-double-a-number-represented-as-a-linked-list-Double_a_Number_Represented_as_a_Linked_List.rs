/**
 * [P002816] Double a Number Represented as a Linked List
 *
 * <p>给你一个 <strong>非空</strong> 链表的头节点 <code>head</code> ，表示一个不含前导零的非负数整数。</p>

<p>将链表 <strong>翻倍</strong> 后，返回头节点<em> </em><code>head</code><em> </em>。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/05/28/example.png" style="width: 401px; height: 81px;" />
<pre>
<strong>输入：</strong>head = [1,8,9]
<strong>输出：</strong>[3,7,8]
<strong>解释：</strong>上图中给出的链表，表示数字 189 。返回的链表表示数字 189 * 2 = 378 。</pre>

<p><strong class="example">示例 2：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/05/28/example2.png" style="width: 401px; height: 81px;" />
<pre>
<strong>输入：</strong>head = [9,9,9]
<strong>输出：</strong>[1,9,9,8]
<strong>解释：</strong>上图中给出的链表，表示数字 999 。返回的链表表示数字 999 * 2 = 1998 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li>链表中节点的数目在范围 <code>[1, 10<sup>4</sup>]</code> 内</li>
	<li><font face="monospace"><code>0 &lt;= Node.val &lt;= 9</code></font></li>
	<li>生成的输入满足：链表表示一个不含前导零的数字，除了数字 <code>0</code> 本身。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/double-a-number-represented-as-a-linked-list/
// discuss: https://leetcode.com/problems/double-a-number-represented-as-a-linked-list/discuss/

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
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002816() {
    }
}