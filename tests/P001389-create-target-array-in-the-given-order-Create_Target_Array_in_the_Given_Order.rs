/**
 * [P001389] Create Target Array in the Given Order
 *
 * <p>给你两个整数数组 <code>nums</code> 和 <code>index</code>。你需要按照以下规则创建目标数组：</p>

<ul>
	<li>目标数组 <code>target</code> 最初为空。</li>
	<li>按从左到右的顺序依次读取 <code>nums[i]</code> 和 <code>index[i]</code>，在 <code>target</code> 数组中的下标 <code>index[i]</code> 处插入值 <code>nums[i]</code> 。</li>
	<li>重复上一步，直到在 <code>nums</code> 和 <code>index</code> 中都没有要读取的元素。</li>
</ul>

<p>请你返回目标数组。</p>

<p>题目保证数字插入位置总是存在。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>nums = [0,1,2,3,4], index = [0,1,2,2,1]
<strong>输出：</strong>[0,4,1,3,2]
<strong>解释：</strong>
nums       index     target
0            0        [0]
1            1        [0,1]
2            2        [0,1,2]
3            2        [0,1,3,2]
4            1        [0,4,1,3,2]
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>nums = [1,2,3,4,0], index = [0,1,2,3,0]
<strong>输出：</strong>[0,1,2,3,4]
<strong>解释：</strong>
nums       index     target
1            0        [1]
2            1        [1,2]
3            2        [1,2,3]
4            3        [1,2,3,4]
0            0        [0,1,2,3,4]
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>nums = [1], index = [0]
<strong>输出：</strong>[1]
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length, index.length &lt;= 100</code></li>
	<li><code>nums.length == index.length</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 100</code></li>
	<li><code>0 &lt;= index[i] &lt;= i</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/create-target-array-in-the-given-order/
// discuss: https://leetcode.com/problems/create-target-array-in-the-given-order/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001389() {
    }
}