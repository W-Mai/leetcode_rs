/**
 * [P002294] Partition Array Such That Maximum Difference Is K
 *
 * <p>给你一个整数数组 <code>nums</code> 和一个整数 <code>k</code> 。你可以将 <code>nums</code> 划分成一个或多个 <strong>子序列</strong> ，使 <code>nums</code> 中的每个元素都 <strong>恰好</strong> 出现在一个子序列中。</p>

<p>在满足每个子序列中最大值和最小值之间的差值最多为 <code>k</code> 的前提下，返回需要划分的 <strong>最少</strong> 子序列数目。</p>

<p><strong>子序列</strong> 本质是一个序列，可以通过删除另一个序列中的某些元素（或者不删除）但不改变剩下元素的顺序得到。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>nums = [3,6,1,2,5], k = 2
<strong>输出：</strong>2
<strong>解释：</strong>
可以将 nums 划分为两个子序列 [3,1,2] 和 [6,5] 。
第一个子序列中最大值和最小值的差值是 3 - 1 = 2 。
第二个子序列中最大值和最小值的差值是 6 - 5 = 1 。
由于创建了两个子序列，返回 2 。可以证明需要划分的最少子序列数目就是 2 。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,2,3], k = 1
<strong>输出：</strong>2
<strong>解释：</strong>
可以将 nums 划分为两个子序列 [1,2] 和 [3] 。
第一个子序列中最大值和最小值的差值是 2 - 1 = 1 。
第二个子序列中最大值和最小值的差值是 3 - 3 = 0 。
由于创建了两个子序列，返回 2 。注意，另一种最优解法是将 nums 划分成子序列 [1] 和 [2,3] 。
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>nums = [2,2,4,5], k = 0
<strong>输出：</strong>3
<strong>解释：</strong>
可以将 nums 划分为三个子序列 [2,2]、[4] 和 [5] 。
第一个子序列中最大值和最小值的差值是 2 - 2 = 0 。
第二个子序列中最大值和最小值的差值是 4 - 4 = 0 。
第三个子序列中最大值和最小值的差值是 5 - 5 = 0 。
由于创建了三个子序列，返回 3 。可以证明需要划分的最少子序列数目就是 3 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= k &lt;= 10<sup>5</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/
// discuss: https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002294() {
    }
}