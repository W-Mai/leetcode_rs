/**
 * [P000380] Insert Delete GetRandom O(1)
 *
 * <p>实现<code>RandomizedSet</code> 类：</p>

<div class="original__bRMd">
<div>
<ul>
	<li><code>RandomizedSet()</code> 初始化 <code>RandomizedSet</code> 对象</li>
	<li><code>bool insert(int val)</code> 当元素 <code>val</code> 不存在时，向集合中插入该项，并返回 <code>true</code> ；否则，返回 <code>false</code> 。</li>
	<li><code>bool remove(int val)</code> 当元素 <code>val</code> 存在时，从集合中移除该项，并返回 <code>true</code> ；否则，返回 <code>false</code> 。</li>
	<li><code>int getRandom()</code> 随机返回现有集合中的一项（测试用例保证调用此方法时集合中至少存在一个元素）。每个元素应该有 <strong>相同的概率</strong> 被返回。</li>
</ul>

<p>你必须实现类的所有函数，并满足每个函数的 <strong>平均</strong> 时间复杂度为 <code>O(1)</code> 。</p>

<p>&nbsp;</p>

<p><strong>示例：</strong></p>

<pre>
<strong>输入</strong>
["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert", "getRandom"]
[[], [1], [2], [2], [], [1], [2], []]
<strong>输出</strong>
[null, true, false, true, 2, true, false, 2]

<strong>解释</strong>
RandomizedSet randomizedSet = new RandomizedSet();
randomizedSet.insert(1); // 向集合中插入 1 。返回 true 表示 1 被成功地插入。
randomizedSet.remove(2); // 返回 false ，表示集合中不存在 2 。
randomizedSet.insert(2); // 向集合中插入 2 。返回 true 。集合现在包含 [1,2] 。
randomizedSet.getRandom(); // getRandom 应随机返回 1 或 2 。
randomizedSet.remove(1); // 从集合中移除 1 ，返回 true 。集合现在包含 [2] 。
randomizedSet.insert(2); // 2 已在集合中，所以返回 false 。
randomizedSet.getRandom(); // 由于 2 是集合中唯一的数字，getRandom 总是返回 2 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>-2<sup>31</sup> &lt;= val &lt;= 2<sup>31</sup> - 1</code></li>
	<li>最多调用 <code>insert</code>、<code>remove</code> 和 <code>getRandom</code> 函数 <code>2 *&nbsp;</code><code>10<sup>5</sup></code> 次</li>
	<li>在调用 <code>getRandom</code> 方法时，数据结构中 <strong>至少存在一个</strong> 元素。</li>
</ul>
</div>
</div>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/insert-delete-getrandom-o1/
// discuss: https://leetcode.com/problems/insert-delete-getrandom-o1/discuss/

// >>> SUBMISSION CODES START HERE <<<

struct RandomizedSet {

}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {

    }
    
    fn insert(&self, val: i32) -> bool {

    }
    
    fn remove(&self, val: i32) -> bool {

    }
    
    fn get_random(&self) -> i32 {

    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000380() {
    }
}