/**
 * [P000641] Design Circular Deque
 *
 * <p>设计实现双端队列。</p>

<p>实现 <code>MyCircularDeque</code> 类:</p>

<ul>
	<li><code>MyCircularDeque(int k)</code>&nbsp;：构造函数,双端队列最大为 <code>k</code> 。</li>
	<li><code>boolean insertFront()</code>：将一个元素添加到双端队列头部。 如果操作成功返回 <code>true</code>&nbsp;，否则返回 <code>false</code> 。</li>
	<li><code>boolean insertLast()</code>&nbsp;：将一个元素添加到双端队列尾部。如果操作成功返回 <code>true</code>&nbsp;，否则返回 <code>false</code> 。</li>
	<li><code>boolean deleteFront()</code>&nbsp;：从双端队列头部删除一个元素。 如果操作成功返回 <code>true</code>&nbsp;，否则返回 <code>false</code> 。</li>
	<li><code>boolean deleteLast()</code>&nbsp;：从双端队列尾部删除一个元素。如果操作成功返回 <code>true</code>&nbsp;，否则返回 <code>false</code> 。</li>
	<li><code>int getFront()</code>&nbsp;)：从双端队列头部获得一个元素。如果双端队列为空，返回 <code>-1</code>&nbsp;。</li>
	<li><code>int getRear()</code>&nbsp;：获得双端队列的最后一个元素。&nbsp;如果双端队列为空，返回 <code>-1</code> 。</li>
	<li><code>boolean isEmpty()</code>&nbsp;：若双端队列为空，则返回&nbsp;<code>true</code>&nbsp;，否则返回 <code>false</code> &nbsp;。</li>
	<li><code>boolean isFull()</code>&nbsp;：若双端队列满了，则返回&nbsp;<code>true</code>&nbsp;，否则返回 <code>false</code> 。</li>
</ul>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入</strong>
["MyCircularDeque", "insertLast", "insertLast", "insertFront", "insertFront", "getRear", "isFull", "deleteLast", "insertFront", "getFront"]
[[3], [1], [2], [3], [4], [], [], [], [4], []]
<strong>输出</strong>
[null, true, true, true, false, 2, true, true, true, 4]

<strong>解释</strong>
MyCircularDeque circularDeque = new MycircularDeque(3); // 设置容量大小为3
circularDeque.insertLast(1);			        // 返回 true
circularDeque.insertLast(2);			        // 返回 true
circularDeque.insertFront(3);			        // 返回 true
circularDeque.insertFront(4);			        // 已经满了，返回 false
circularDeque.getRear();  				// 返回 2
circularDeque.isFull();				        // 返回 true
circularDeque.deleteLast();			        // 返回 true
circularDeque.insertFront(4);			        // 返回 true
circularDeque.getFront();				// 返回 4
&nbsp;</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= 1000</code></li>
	<li><code>0 &lt;= value &lt;= 1000</code></li>
	<li><code>insertFront</code>,&nbsp;<code>insertLast</code>,&nbsp;<code>deleteFront</code>,&nbsp;<code>deleteLast</code>,&nbsp;<code>getFront</code>,&nbsp;<code>getRear</code>,&nbsp;<code>isEmpty</code>,&nbsp;<code>isFull</code>&nbsp; 调用次数不大于&nbsp;<code>2000</code>&nbsp;次</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/design-circular-deque/
// discuss: https://leetcode.com/problems/design-circular-deque/discuss/

// >>> SUBMISSION CODES START HERE <<<

struct MyCircularDeque {

}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {

    fn new(k: i32) -> Self {

    }
    
    fn insert_front(&self, value: i32) -> bool {

    }
    
    fn insert_last(&self, value: i32) -> bool {

    }
    
    fn delete_front(&self) -> bool {

    }
    
    fn delete_last(&self) -> bool {

    }
    
    fn get_front(&self) -> i32 {

    }
    
    fn get_rear(&self) -> i32 {

    }
    
    fn is_empty(&self) -> bool {

    }
    
    fn is_full(&self) -> bool {

    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000641() {
    }
}