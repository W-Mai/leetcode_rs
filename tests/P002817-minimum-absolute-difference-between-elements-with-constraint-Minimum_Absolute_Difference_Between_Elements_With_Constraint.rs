/**
 * [P002817] Minimum Absolute Difference Between Elements With Constraint
 *
 * <p>给你一个下标从 <strong>0</strong>&nbsp;开始的整数数组&nbsp;<code>nums</code>&nbsp;和一个整数&nbsp;<code>x</code>&nbsp;。</p>

<p>请你找到数组中下标距离至少为 <code>x</code>&nbsp;的两个元素的 <strong>差值绝对值</strong>&nbsp;的 <strong>最小值</strong>&nbsp;。</p>

<p>换言之，请你找到两个下标&nbsp;<code>i</code> 和&nbsp;<code>j</code>&nbsp;，满足&nbsp;<code>abs(i - j) &gt;= x</code> 且&nbsp;<code>abs(nums[i] - nums[j])</code>&nbsp;的值最小。</p>

<p>请你返回一个整数，表示下标距离至少为 <code>x</code>&nbsp;的两个元素之间的差值绝对值的 <strong>最小值</strong>&nbsp;。</p>

<p>&nbsp;</p>

<p><b>示例 1：</b></p>

<pre>
<b>输入：</b>nums = [4,3,2,4], x = 2
<b>输出：</b>0
<b>解释：</b>我们选择 nums[0] = 4 和 nums[3] = 4 。
它们下标距离满足至少为 2 ，差值绝对值为最小值 0 。
0 是最优解。
</pre>

<p><strong class="example">示例 2：</strong></p>

<pre>
<b>输入：</b>nums = [5,3,2,10,15], x = 1
<b>输出：</b>1
<b>解释：</b>我们选择 nums[1] = 3 和 nums[2] = 2 。
它们下标距离满足至少为 1 ，差值绝对值为最小值 1 。
1 是最优解。
</pre>

<p><strong class="example">示例 3：</strong></p>

<pre>
<b>输入：</b>nums = [1,2,3,4], x = 3
<b>输出：</b>3
<strong>解释：</strong>我们选择 nums[0] = 1 和 nums[3] = 4 。
它们下标距离满足至少为 3 ，差值绝对值为最小值 3 。
3 是最优解。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= x &lt; nums.length</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/minimum-absolute-difference-between-elements-with-constraint/
// discuss: https://leetcode.com/problems/minimum-absolute-difference-between-elements-with-constraint/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn min_absolute_difference(nums: Vec<i32>, x: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002817() {
    }
}