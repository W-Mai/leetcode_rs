/**
 * [P002640] Find the Score of All Prefixes of an Array
 *
 * <p>定义一个数组 <code>arr</code>&nbsp;的 <strong>转换数组</strong>&nbsp;<code>conver</code>&nbsp;为：</p>

<ul>
	<li><code>conver[i] = arr[i] + max(arr[0..i])</code>，其中&nbsp;<code>max(arr[0..i])</code>&nbsp;是满足 <code>0 &lt;= j &lt;= i</code>&nbsp;的所有&nbsp;<code>arr[j]</code>&nbsp;中的最大值。</li>
</ul>

<p>定义一个数组 <code>arr</code>&nbsp;的 <strong>分数</strong>&nbsp;为 <code>arr</code>&nbsp;转换数组中所有元素的和。</p>

<p>给你一个下标从 <strong>0</strong>&nbsp;开始长度为 <code>n</code>&nbsp;的整数数组&nbsp;<code>nums</code>&nbsp;，请你返回一个长度为 <code>n</code>&nbsp;的数组<em>&nbsp;</em><code>ans</code>&nbsp;，其中&nbsp;<code>ans[i]</code>是前缀&nbsp;<code>nums[0..i]</code>&nbsp;的分数。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><b>输入：</b>nums = [2,3,7,5,10]
<b>输出：</b>[4,10,24,36,56]
<b>解释：</b>
对于前缀 [2] ，转换数组为 [4] ，所以分数为 4 。
对于前缀 [2, 3] ，转换数组为 [4, 6] ，所以分数为 10 。
对于前缀 [2, 3, 7] ，转换数组为 [4, 6, 14] ，所以分数为 24 。
对于前缀 [2, 3, 7, 5] ，转换数组为 [4, 6, 14, 12] ，所以分数为 36 。
对于前缀 [2, 3, 7, 5, 10] ，转换数组为 [4, 6, 14, 12, 20] ，所以分数为 56 。
</pre>

<p><strong>示例 2：</strong></p>

<pre><b>输入：</b>nums = [1,1,2,4,8,16]
<b>输出：</b>[2,4,8,16,32,64]
<b>解释：</b>
对于前缀 [1] ，转换数组为 [2] ，所以分数为 2 。
对于前缀 [1, 1]，转换数组为 [2, 2] ，所以分数为 4 。
对于前缀 [1, 1, 2]，转换数组为 [2, 2, 4] ，所以分数为 8 。
对于前缀 [1, 1, 2, 4]，转换数组为 [2, 2, 4, 8] ，所以分数为 16 。
对于前缀 [1, 1, 2, 4, 8]，转换数组为 [2, 2, 4, 8, 16] ，所以分数为 32 。
对于前缀 [1, 1, 2, 4, 8, 16]，转换数组为 [2, 2, 4, 8, 16, 32] ，所以分数为 64 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/find-the-score-of-all-prefixes-of-an-array/
// discuss: https://leetcode.com/problems/find-the-score-of-all-prefixes-of-an-array/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002640() {
    }
}