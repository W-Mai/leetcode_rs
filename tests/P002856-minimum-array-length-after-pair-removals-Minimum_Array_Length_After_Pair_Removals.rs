/**
 * [P002856] Minimum Array Length After Pair Removals
 *
 * <p>给你一个下标从 <strong>0</strong> 开始的 <strong>非递减</strong> 整数数组&nbsp;<code>nums</code>&nbsp;。</p>

<p>你可以执行以下操作任意次：</p>

<ul>
	<li>选择 <strong>两个&nbsp;</strong>下标&nbsp;<code>i</code> 和&nbsp;<code>j</code>&nbsp;，满足&nbsp;<code>i &lt; j</code>&nbsp;且&nbsp;<code>nums[i] &lt; nums[j]</code>&nbsp;。</li>
	<li>将 <code>nums</code>&nbsp;中下标在&nbsp;<code>i</code> 和&nbsp;<code>j</code>&nbsp;处的元素删除。剩余元素按照原来的顺序组成新的数组，下标也重新从 <strong>0</strong>&nbsp;开始编号。</li>
</ul>

<p>请你返回一个整数，表示执行以上操作任意次后（可以执行 <strong>0</strong> 次），<code>nums</code>&nbsp;数组的 <strong>最小</strong>&nbsp;数组长度。</p>

<p>请注意，<code>nums</code> 数组是按&nbsp;<strong>非降序&nbsp;</strong>排序的。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,3,4,9]
<strong>输出：</strong>0
<strong>解释：</strong>一开始，nums = [1, 3, 4, 9] 。
第一次操作，我们选择下标 0 和 1 ，满足 nums[0] &lt; nums[1] &lt;=&gt; 1 &lt; 3 。
删除下标 0 和 1 处的元素，nums 变成 [4, 9] 。
下一次操作，我们选择下标 0 和 1 ，满足 nums[0] &lt; nums[1] &lt;=&gt; 4 &lt; 9 。
删除下标 0 和 1 处的元素，nums 变成空数组 [] 。
所以，可以得到的最小数组长度为 0 。</pre>

<p><strong class="example">示例 2：</strong></p>

<pre>
<strong>输入：</strong>nums = [2,3,6,9]
<strong>输出：</strong>0
<strong>解释：</strong>一开始，nums = [2, 3, 6, 9] 。
第一次操作，我们选择下标 0 和 2 ，满足 nums[0] &lt; nums[2] &lt;=&gt; 2 &lt; 6 。
删除下标 0 和 2 处的元素，nums 变成 [3, 9] 。
下一次操作，我们选择下标 0 和 1 ，满足 nums[0] &lt; nums[1] &lt;=&gt; 3 &lt; 9 。
删除下标 0 和 1 处的元素，nums 变成空数组 [] 。
所以，可以得到的最小数组长度为 0 。
</pre>

<p><strong class="example">示例 3：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,1,2]
<strong>输出：</strong>1
<strong>解释：</strong>一开始，nums = [1, 1, 2] 。
第一次操作，我们选择下标 0 和 2 ，满足 nums[0] &lt; nums[2] &lt;=&gt; 1 &lt; 2 。
删除下标 0 和 2 处的元素，nums 变成 [1] 。
无法对数组再执行操作。
所以，可以得到的最小数组长度为 1 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>nums</code>&nbsp;是 <strong>非递减</strong>&nbsp;数组。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/minimum-array-length-after-pair-removals/
// discuss: https://leetcode.com/problems/minimum-array-length-after-pair-removals/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn min_length_after_removals(nums: Vec<i32>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002856() {
    }
}