/**
 * [P002681] Power of Heroes
 *
 * <p>给你一个下标从 <strong>0</strong>&nbsp;开始的整数数组&nbsp;<code>nums</code>&nbsp;，它表示英雄的能力值。如果我们选出一部分英雄，这组英雄的 <strong>力量</strong>&nbsp;定义为：</p>

<ul>
	<li><code>i<sub>0</sub></code>&nbsp;，<code>i<sub>1</sub></code>&nbsp;，<span style="">... </span><code><span style="">i<sub>k</sub></span></code><span style="">&nbsp;</span>表示这组英雄在数组中的下标。那么这组英雄的力量为&nbsp;<code><font face="monospace">max(nums[</font>i<sub>0</sub><font face="monospace">],nums[</font>i<sub>1</sub><font face="monospace">] ... nums[</font><span style="font-size:10.8333px">i<sub>k</sub></span><font face="monospace">])<sup>2</sup> * min(nums[</font>i<sub>0</sub><font face="monospace">],nums[</font>i<sub>1</sub><font face="monospace">] ... nums[</font><span style="font-size:10.8333px">i<sub>k</sub></span><font face="monospace">])</font></code> 。</li>
</ul>

<p>请你返回所有可能的 <strong>非空</strong> 英雄组的 <strong>力量</strong> 之和。由于答案可能非常大，请你将结果对&nbsp;<code>10<sup>9 </sup>+ 7</code>&nbsp;<strong>取余。</strong></p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<b>输入：</b>nums = [2,1,4]
<b>输出：</b>141
<b>解释：</b>
第 1&nbsp;组：[2] 的力量为 2<sup>2</sup>&nbsp;* 2 = 8 。
第 2&nbsp;组：[1] 的力量为 1<sup>2</sup> * 1 = 1 。
第 3&nbsp;组：[4] 的力量为 4<sup>2</sup> * 4 = 64 。
第 4&nbsp;组：[2,1] 的力量为 2<sup>2</sup> * 1 = 4 。
第 5 组：[2,4] 的力量为 4<sup>2</sup> * 2 = 32 。
第 6&nbsp;组：[1,4] 的力量为 4<sup>2</sup> * 1 = 16 。
第​ ​​​​​​7&nbsp;组：[2,1,4] 的力量为 4<sup>2</sup>​​​​​​​ * 1 = 16 。
所有英雄组的力量之和为 8 + 1 + 64 + 4 + 32 + 16 + 16 = 141 。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<b>输入：</b>nums = [1,1,1]
<b>输出：</b>7
<b>解释：</b>总共有 7 个英雄组，每一组的力量都是 1 。所以所有英雄组的力量之和为 7 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/power-of-heroes/
// discuss: https://leetcode.com/problems/power-of-heroes/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn sum_of_power(nums: Vec<i32>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002681() {
    }
}