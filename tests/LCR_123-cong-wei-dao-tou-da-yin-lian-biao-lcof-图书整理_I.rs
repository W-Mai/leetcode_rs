/**
 * [LCR 123] 图书整理 I
 *
 * <p>书店店员有一张链表形式的书单，每个节点代表一本书，节点中的值表示书的编号。为更方便整理书架，店员需要将书单倒过来排列，就可以从最后一本书开始整理，逐一将书放回到书架上。请倒序返回这个书单链表。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>head = [3,6,4,1]

<strong>输出：</strong>[1,4,6,3]
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<p><code>0 &lt;= 链表长度 &lt;= 10000</code></p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/cong-wei-dao-tou-da-yin-lian-biao-lcof/
// discuss: https://leetcode.com/problems/cong-wei-dao-tou-da-yin-lian-biao-lcof/discuss/

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
    pub fn reverse_book_list(head: Option<Box<ListNode>>) -> Vec<i32> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCR 123() {
    }
}