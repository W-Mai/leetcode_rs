/**
 * [P000480] Sliding Window Median
 *
 * <p>中位数是有序序列最中间的那个数。如果序列的长度是偶数，则没有最中间的数；此时中位数是最中间的两个数的平均数。</p>

<p>例如：</p>

<ul>
	<li><code>[2,3,4]</code>，中位数是 <code>3</code></li>
	<li><code>[2,3]</code>，中位数是 <code>(2 + 3) / 2 = 2.5</code></li>
</ul>

<p>给你一个数组 <em>nums</em>，有一个长度为 <em>k</em> 的窗口从最左端滑动到最右端。窗口中有 <em>k</em> 个数，每次窗口向右移动 <em>1</em> 位。你的任务是找出每次窗口移动后得到的新窗口中元素的中位数，并输出由它们组成的数组。</p>

<p> </p>

<p><strong>示例：</strong></p>

<p>给出 <em>nums</em> = <code>[1,3,-1,-3,5,3,6,7]</code>，以及 <em>k</em> = 3。</p>

<pre>
窗口位置                      中位数
---------------               -----
[1  3  -1] -3  5  3  6  7       1
 1 [3  -1  -3] 5  3  6  7      -1
 1  3 [-1  -3  5] 3  6  7      -1
 1  3  -1 [-3  5  3] 6  7       3
 1  3  -1  -3 [5  3  6] 7       5
 1  3  -1  -3  5 [3  6  7]      6
</pre>

<p> 因此，返回该滑动窗口的中位数数组 <code>[1,-1,-1,3,5,6]</code>。</p>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li>你可以假设 <code>k</code> 始终有效，即：<code>k</code> 始终小于等于输入的非空数组的元素个数。</li>
	<li>与真实值误差在 <code>10 ^ -5</code> 以内的答案将被视作正确答案。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/sliding-window-median/
// discuss: https://leetcode.com/problems/sliding-window-median/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000480() {
    }
}