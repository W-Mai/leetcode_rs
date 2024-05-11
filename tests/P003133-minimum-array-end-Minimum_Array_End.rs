/**
 * [P003133] Minimum Array End
 *
 * <p>给你两个整数 <code>n</code> 和 <code>x</code> 。你需要构造一个长度为 <code>n</code> 的 <strong>正整数 </strong>数组 <code>nums</code> ，对于所有 <code>0 &lt;= i &lt; n - 1</code> ，满足 <code>nums[i + 1]</code><strong> 大于 </strong><code>nums[i]</code> ，并且数组 <code>nums</code> 中所有元素的按位 <code>AND</code> 运算结果为 <code>x</code> 。</p>

<p>返回 <code>nums[n - 1]</code> 可能的<strong> 最小 </strong>值。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<div class="example-block">
<p><strong>输入：</strong><span class="example-io">n = 3, x = 4</span></p>

<p><strong>输出：</strong><span class="example-io">6</span></p>

<p><strong>解释：</strong></p>

<p>数组 <code>nums</code> 可以是 <code>[4,5,6]</code> ，最后一个元素为 <code>6</code> 。</p>
</div>

<p><strong class="example">示例 2：</strong></p>

<div class="example-block">
<p><strong>输入：</strong><span class="example-io">n = 2, x = 7</span></p>

<p><strong>输出：</strong><span class="example-io">15</span></p>

<p><strong>解释：</strong></p>

<p>数组 <code>nums</code> 可以是 <code>[7,15]</code> ，最后一个元素为 <code>15</code> 。</p>
</div>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= n, x &lt;= 10<sup>8</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/minimum-array-end/
// discuss: https://leetcode.com/problems/minimum-array-end/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P003133() {
    }
}