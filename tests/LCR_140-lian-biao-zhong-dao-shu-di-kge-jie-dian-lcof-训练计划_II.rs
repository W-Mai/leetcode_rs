/**
 * [LCR 140] 训练计划 II
 *
 * <p>给定一个头节点为 <code>head</code> 的链表用于记录一系列核心肌群训练项目编号，请查找并返回倒数第 <code>cnt</code> 个训练项目编号。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>head = [2,4,7,8], cnt = 1
<strong>输出：</strong>8</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= head.length &lt;= 100</code></li>
	<li><code>0 &lt;= head[i] &lt;= 100</code></li>
	<li><code>1 &lt;= cnt &lt;= head.length</code></li>
</ul>

<p>&nbsp;</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/lian-biao-zhong-dao-shu-di-kge-jie-dian-lcof/
// discuss: https://leetcode.com/problems/lian-biao-zhong-dao-shu-di-kge-jie-dian-lcof/discuss/

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
    pub fn training_plan(head: Option<Box<ListNode>>, cnt: i32) -> Option<Box<ListNode>> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCR 140() {
    }
}