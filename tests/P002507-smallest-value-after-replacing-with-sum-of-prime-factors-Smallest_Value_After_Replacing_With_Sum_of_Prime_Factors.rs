/**
 * [P002507] Smallest Value After Replacing With Sum of Prime Factors
 *
 * <p>给你一个正整数 <code>n</code> 。</p>

<p>请你将 <code>n</code> 的值替换为 <code>n</code> 的 <strong>质因数</strong> 之和，重复这一过程。</p>

<ul>
	<li>注意，如果 <code>n</code> 能够被某个质因数多次整除，则在求和时，应当包含这个质因数同样次数。</li>
</ul>

<p>返回<em> </em><code>n</code><em> </em>可以取到的最小值。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>n = 15
<strong>输出：</strong>5
<strong>解释：</strong>最开始，n = 15 。
15 = 3 * 5 ，所以 n 替换为 3 + 5 = 8 。
8 = 2 * 2 * 2 ，所以 n 替换为 2 + 2 + 2 = 6 。
6 = 2 * 3 ，所以 n 替换为 2 + 3 = 5 。
5 是 n 可以取到的最小值。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>n = 3
<strong>输出：</strong>3
<strong>解释：</strong>最开始，n = 3 。
3 是 n 可以取到的最小值。</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/smallest-value-after-replacing-with-sum-of-prime-factors/
// discuss: https://leetcode.com/problems/smallest-value-after-replacing-with-sum-of-prime-factors/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn smallest_value(n: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002507() {
    }
}