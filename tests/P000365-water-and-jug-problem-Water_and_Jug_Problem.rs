/**
 * [P000365] Water and Jug Problem
 *
 * <p>有两个水壶，容量分别为&nbsp;<code>x</code>&nbsp;和 <code>y</code> 升。水的供应是无限的。确定是否有可能使用这两个壶准确得到&nbsp;<code>target</code>&nbsp;升。</p>

<p>你可以：</p>

<ul>
	<li>装满任意一个水壶</li>
	<li>清空任意一个水壶</li>
	<li>将水从一个水壶倒入另一个水壶，直到接水壶已满，或倒水壶已空。</li>
</ul>

<p>&nbsp;</p>

<p><strong>示例 1:</strong>&nbsp;</p>

<pre>
<strong>输入:</strong> x = 3,y = 5,target = 4
<strong>输出:</strong> true
<strong>解释：
</strong>按照以下步骤操作，以达到总共 4 升水：
1. 装满 5 升的水壶(0, 5)。
2. 把 5 升的水壶倒进 3 升的水壶，留下 2 升(3, 2)。
3. 倒空 3 升的水壶(0, 2)。
4. 把 2 升水从 5 升的水壶转移到 3 升的水壶(2, 0)。
5. 再次加满 5 升的水壶(2, 5)。
6. 从 5 升的水壶向 3 升的水壶倒水直到 3 升的水壶倒满。5 升的水壶里留下了 4 升水(3, 4)。
7. 倒空 3 升的水壶。现在，5 升的水壶里正好有 4 升水(0, 4)。
参考：来自著名的&nbsp;<a href="https://www.youtube.com/watch?v=BVtQNK_ZUJg"><em>"Die Hard"</em></a></pre>

<p><strong>示例 2:</strong></p>

<pre>
<strong>输入:</strong> x = 2, y = 6, target = 5
<strong>输出:</strong> false
</pre>

<p><strong>示例 3:</strong></p>

<pre>
<strong>输入:</strong> x = 1, y = 2, target = 3
<strong>输出:</strong> true
<b>解释：</b>同时倒满两个水壶。现在两个水壶中水的总量等于 3。</pre>

<p>&nbsp;</p>

<p><strong>提示:</strong></p>

<ul>
	<li><code>1 &lt;= x, y, target &lt;= 10<sup>3</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/water-and-jug-problem/
// discuss: https://leetcode.com/problems/water-and-jug-problem/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, target: i32) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000365() {
    }
}