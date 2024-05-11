/**
 * [LCR 147] 最小栈
 *
 * <p>请你设计一个 <strong>最小栈</strong> 。它提供 <code>push</code> ，<code>pop</code> ，<code>top</code> 操作，并能在常数时间内检索到最小元素的栈。</p>

<p>&nbsp;</p>

<p>实现 <code>MinStack</code> 类:</p>

<ul>
	<li><code>MinStack()</code> 初始化堆栈对象。</li>
	<li><code>void push(int val)</code> 将元素val推入堆栈。</li>
	<li><code>void pop()</code> 删除堆栈顶部的元素。</li>
	<li><code>int top()</code> 获取堆栈顶部的元素。</li>
	<li><code>int getMin()</code> 获取堆栈中的最小元素。</li>
</ul>

<p>&nbsp;</p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入：</strong>
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[2],[-3],[],[],[],[]]

<strong>输出：</strong>
[null,null,null,null,-3,null,2,-2]

<strong>解释：</strong>
MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(2);
minStack.push(-3);
minStack.getMin(); &nbsp; --&gt; 返回 -3.
minStack.pop();
minStack.top(); &nbsp; &nbsp; &nbsp;--&gt; 返回 2.
minStack.getMin(); &nbsp; --&gt; 返回 -2.
</pre>

<p>&nbsp;</p>

<p><strong>&nbsp;<br />
提示：</strong></p>

<ul>
	<li><code>-2<sup>31</sup> &lt;= val &lt;= 2<sup>31</sup> - 1</code></li>
	<li><code>pop</code>、<code>top</code> 和 <code>getMin</code> 操作总是在 <strong>非空栈</strong> 上调用</li>
	<li><code>push</code>、<code>pop</code>、<code>top</code> 和 <code>getMin</code> 最多被调用 <code>3 * 10<sup>4</sup></code> 次</li>
</ul>

<p>&nbsp;</p>

<p>注意：本题与主站 155 题相同：<a href="https://leetcode-cn.com/problems/min-stack/">https://leetcode-cn.com/problems/min-stack/</a></p>

<p>&nbsp;</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/bao-han-minhan-shu-de-zhan-lcof/
// discuss: https://leetcode.com/problems/bao-han-minhan-shu-de-zhan-lcof/discuss/

// >>> SUBMISSION CODES START HERE <<<

struct MinStack {

}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {

    }
    
    fn push(&self, x: i32) {

    }
    
    fn pop(&self) {

    }
    
    fn top(&self) -> i32 {

    }
    
    fn get_min(&self) -> i32 {

    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCR 147() {
    }
}