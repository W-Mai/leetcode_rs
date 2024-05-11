/**
 * [P001053] Previous Permutation With One Swap
 *
 * <p>给你一个正整数数组 <code>arr</code>（可能存在重复的元素），请你返回可在&nbsp;<strong>一次交换</strong>（交换两数字 <code>arr[i]</code> 和 <code>arr[j]</code> 的位置）后得到的、按<span data-keyword="lexicographically-smaller-string-alien">字典序</span>排列小于 <code>arr</code> 的最大排列。</p>

<p>如果无法这么操作，就请返回原数组。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>arr = [3,2,1]
<strong>输出：</strong>[3,1,2]
<strong>解释：</strong>交换 2 和 1
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>arr = [1,1,5]
<strong>输出：</strong>[1,1,5]
<strong>解释：</strong>已经是最小排列
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>arr = [1,9,4,6,7]
<strong>输出：</strong>[1,7,4,6,9]
<strong>解释：</strong>交换 9 和 7
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= arr[i] &lt;= 10<sup>4</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/previous-permutation-with-one-swap/
// discuss: https://leetcode.com/problems/previous-permutation-with-one-swap/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001053() {
    }
}