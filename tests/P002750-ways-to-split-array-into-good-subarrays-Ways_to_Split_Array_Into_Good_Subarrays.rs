/**
 * [P002750] Ways to Split Array Into Good Subarrays
 *
 * <p>给你一个二元数组 <code>nums</code> 。</p>

<p>如果数组中的某个子数组 <strong>恰好</strong> 只存在 <strong>一</strong> 个值为 <code>1</code> 的元素，则认为该子数组是一个 <strong>好子数组</strong> 。</p>

<p>请你统计将数组 <code>nums</code> 划分成若干 <strong>好子数组</strong> 的方法数，并以整数形式返回。由于数字可能很大，返回其对 <code>10<sup>9</sup> + 7</code> <strong>取余 </strong>之后的结果。</p>

<p>子数组是数组中的一个连续 <strong>非空</strong> 元素序列。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>nums = [0,1,0,0,1]
<strong>输出：</strong>3
<strong>解释：</strong>存在 3 种可以将 nums 划分成若干好子数组的方式：
- [0,1] [0,0,1]
- [0,1,0] [0,1]
- [0,1,0,0] [1]
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>nums = [0,1,0]
<strong>输出：</strong>1
<strong>解释：</strong>存在 1 种可以将 nums 划分成若干好子数组的方式：
- [0,1,0]
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 1</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/ways-to-split-array-into-good-subarrays/
// discuss: https://leetcode.com/problems/ways-to-split-array-into-good-subarrays/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn number_of_good_subarray_splits(nums: Vec<i32>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002750() {
    }
}