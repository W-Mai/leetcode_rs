/**
 * [P001043] Partition Array for Maximum Sum
 *
 * <p>给你一个整数数组 <code>arr</code>，请你将该数组分隔为长度 <strong>最多 </strong>为 k 的一些（连续）子数组。分隔完成后，每个子数组的中的所有值都会变为该子数组中的最大值。</p>

<p>返回将数组分隔变换后能够得到的元素最大和。本题所用到的测试用例会确保答案是一个 32 位整数。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>arr = [1,15,7,9,2,5,10], k = 3
<strong>输出：</strong>84
<strong>解释：</strong>数组变为 [15,15,15,9,10,10,10]</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>arr = [1,4,1,5,7,3,6,1,9,9,3], k = 4
<strong>输出：</strong>83
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>arr = [1], k = 1
<strong>输出：</strong>1
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 500</code></li>
	<li><code>0 &lt;= arr[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= k &lt;= arr.length</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/partition-array-for-maximum-sum/
// discuss: https://leetcode.com/problems/partition-array-for-maximum-sum/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001043() {
    }
}