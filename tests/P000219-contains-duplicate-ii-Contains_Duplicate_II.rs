/**
 * [P000219] Contains Duplicate II
 *
 * <p>给你一个整数数组&nbsp;<code>nums</code> 和一个整数&nbsp;<code>k</code> ，判断数组中是否存在两个 <strong>不同的索引</strong><em>&nbsp;</em><code>i</code>&nbsp;和<em>&nbsp;</em><code>j</code> ，满足 <code>nums[i] == nums[j]</code> 且 <code>abs(i - j) &lt;= k</code> 。如果存在，返回 <code>true</code> ；否则，返回 <code>false</code> 。</p>

<p>&nbsp;</p>

<p><strong>示例&nbsp;1：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,2,3,1], k<em> </em>= 3
<strong>输出：</strong>true</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,0,1,1], k<em> </em>=<em> </em>1
<strong>输出：</strong>true</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>nums = [1,2,3,1,2,3], k<em> </em>=<em> </em>2
<strong>输出：</strong>false</pre>

<p>&nbsp;</p>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= k &lt;= 10<sup>5</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/contains-duplicate-ii/
// discuss: https://leetcode.com/problems/contains-duplicate-ii/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000219() {
    }
}