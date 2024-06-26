/**
 * [P001420] Build Array Where You Can Find The Maximum Exactly K Comparisons
 *
 * <p>给定三个整数 <code>n</code>、<code>m</code> 和 <code>k</code> 。考虑使用下图描述的算法找出正整数数组中最大的元素。</p>

<p><img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/04/19/e.png" style="height: 372px; width: 424px;" /></p>

<p>请你构建一个具有以下属性的数组 <code>arr</code> ：</p>

<ul>
	<li><code>arr</code> 中包含确切的&nbsp;<code>n</code> 个整数。</li>
	<li><code>1 &lt;= arr[i] &lt;= m</code> 其中 <code>(0 &lt;= i &lt; n)</code> 。</li>
	<li>将上面提到的算法应用于 <code>arr</code>&nbsp;之后，<code>search_cost</code> 的值等于 <code>k</code> 。</li>
</ul>

<p>返回在满足上述条件的情况下构建数组 <code>arr</code> 的 <em>方法数量</em>&nbsp;，由于答案可能会很大，所以 <strong>必须</strong> 对 <code>10^9 + 7</code> 取余。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>n = 2, m = 3, k = 1
<strong>输出：</strong>6
<strong>解释：</strong>可能的数组分别为 [1, 1], [2, 1], [2, 2], [3, 1], [3, 2] [3, 3]
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>n = 5, m = 2, k = 3
<strong>输出：</strong>0
<strong>解释：</strong>没有数组可以满足上述条件
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>n = 9, m = 1, k = 1
<strong>输出：</strong>1
<strong>解释：</strong>唯一可能的数组是 [1, 1, 1, 1, 1, 1, 1, 1, 1]
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 50</code></li>
	<li><code>1 &lt;= m &lt;= 100</code></li>
	<li><code>0 &lt;= k &lt;= n</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/
// discuss: https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001420() {
    }
}