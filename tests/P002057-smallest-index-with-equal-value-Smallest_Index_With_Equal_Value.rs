/**
 * [P002057] Smallest Index With Equal Value
 *
 * <p>给你一个下标从 0 开始的整数数组 <code>nums</code> ，返回 <code>nums</code> 中满足<em> </em><code>i mod 10 == nums[i]</code><em> </em>的最小下标 <code>i</code> ；如果不存在这样的下标，返回<em> </em><code>-1</code><em> </em>。</p>

<p><code>x mod y</code> 表示 <code>x</code> 除以 <code>y</code> 的 <strong>余数</strong> 。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>nums = [0,1,2]
<strong>输出：</strong>0
<strong>解释：</strong>
i=0: 0 mod 10 = 0 == nums[0].
i=1: 1 mod 10 = 1 == nums[1].
i=2: 2 mod 10 = 2 == nums[2].
所有下标都满足 i mod 10 == nums[i] ，所以返回最小下标 0
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>nums = [4,3,2,1]
<strong>输出：</strong>2
<strong>解释：</strong>
i=0: 0 mod 10 = 0 != nums[0].
i=1: 1 mod 10 = 1 != nums[1].
i=2: 2 mod 10 = 2 == nums[2].
i=3: 3 mod 10 = 3 != nums[3].
2 唯一一个满足 i mod 10 == nums[i] 的下标
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>nums = [1,2,3,4,5,6,7,8,9,0]
<strong>输出：</strong>-1
<strong>解释：</strong>不存在满足 i mod 10 == nums[i] 的下标
</pre>

<p><strong>示例 4：</strong></p>

<pre><strong>输入：</strong>nums = [2,1,3,5,2]
<strong>输出：</strong>1
<strong>解释：</strong>1 是唯一一个满足 i mod 10 == nums[i] 的下标
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 100</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 9</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/smallest-index-with-equal-value/
// discuss: https://leetcode.com/problems/smallest-index-with-equal-value/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002057() {
    }
}