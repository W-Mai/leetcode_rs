/**
 * [P001383] Maximum Performance of a Team
 *
 * <p>给定两个整数 <code>n</code> 和 <code>k</code>，以及两个长度为 <code>n</code> 的整数数组 <code>speed</code> 和<code> efficiency</code>。现有 <code>n</code> 名工程师，编号从 <code>1</code> 到 <code>n</code>。其中 <code>speed[i]</code>&nbsp;和 <code>efficiency[i]</code>&nbsp;分别代表第 <code>i</code>&nbsp;位工程师的速度和效率。</p>

<p>从这 <code>n</code> 名工程师中最多选择 <code>k</code> 名不同的工程师，使其组成的团队具有最大的团队表现值。</p>

<p><strong>团队表现值</strong>&nbsp;的定义为：一个团队中「所有工程师速度的和」乘以他们「效率值中的最小值」。</p>

<p>请你返回该团队的​​​​​​最大团队表现值，由于答案可能很大，请你返回结果对 <code>10^9 + 7</code> 取余后的结果。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 2
<strong>输出：</strong>60
<strong>解释：</strong>
我们选择工程师 2（speed=10 且 efficiency=4）和工程师 5（speed=5 且 efficiency=7）。他们的团队表现值为 performance = (10 + 5) * min(4, 7) = 60 。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 3
<strong>输出：</strong>68
<strong>解释：
</strong>此示例与第一个示例相同，除了 k = 3 。我们可以选择工程师 1 ，工程师 2 和工程师 5 得到最大的团队表现值。表现值为 performance = (2 + 10 + 5) * min(5, 4, 7) = 68 。
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 4
<strong>输出：</strong>72
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= n &lt;= 10^5</code></li>
	<li><code>speed.length == n</code></li>
	<li><code>efficiency.length == n</code></li>
	<li><code>1 &lt;= speed[i] &lt;= 10^5</code></li>
	<li><code>1 &lt;= efficiency[i] &lt;= 10^8</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/maximum-performance-of-a-team/
// discuss: https://leetcode.com/problems/maximum-performance-of-a-team/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001383() {
    }
}