/**
 * [P001481] Least Number of Unique Integers after K Removals
 *
 * <p>给你一个整数数组 <code>arr</code> 和一个整数 <code>k</code> 。现需要从数组中恰好移除 <code>k</code> 个元素，请找出移除后数组中不同整数的最少数目。</p>

<ol>
</ol>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>arr = [5,5,4], k = 1
<strong>输出：</strong>1
<strong>解释：</strong>移除 1 个 4 ，数组中只剩下 5 一种整数。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>arr = [4,3,1,1,3,3,2], k = 3
<strong>输出：</strong>2
<strong>解释：</strong>先移除 4、2 ，然后再移除两个 1 中的任意 1 个或者三个 3 中的任意 1 个，最后剩下 1 和 3 两种整数。</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= arr.length&nbsp;&lt;= 10^5</code></li>
	<li><code>1 &lt;= arr[i] &lt;= 10^9</code></li>
	<li><code>0 &lt;= k&nbsp;&lt;= arr.length</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/
// discuss: https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001481() {
    }
}