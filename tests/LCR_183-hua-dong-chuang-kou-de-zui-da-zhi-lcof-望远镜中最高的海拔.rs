/**
 * [LCR 183] 望远镜中最高的海拔
 *
 * <p>科技馆内有一台虚拟观景望远镜，它可以用来观测特定纬度地区的地形情况。该纬度的海拔数据记于数组 <code>heights</code> ，其中 <code>heights[i]</code> 表示对应位置的海拔高度。请找出并返回望远镜视野范围 <code>limit</code> 内，可以观测到的最高海拔值。</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>heights = [14,2,27,-5,28,13,39], limit = 3
<strong>输出：</strong>[27,27,28,28,39]
<strong>解释：</strong>
  滑动窗口的位置                最大值
---------------               -----
[14 2 27] -5 28 13 39          27
14 [2 27 -5] 28 13 39          27
14 2 [27 -5 28] 13 39          28
14 2 27 [-5 28 13] 39          28
14 2 27 -5 [28 13 39]          39</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<p>你可以假设输入总是有效的，在输入数组不为空的情况下：</p>

<ul>
	<li><code>1 &lt;= limit &lt;= heights.length</code></li>
	<li><code>-10000 &lt;= heights[i] &lt;= 10000</code></li>
</ul>

<p>注意：本题与主站 239 题相同：<a href="https://leetcode-cn.com/problems/sliding-window-maximum/">https://leetcode-cn.com/problems/sliding-window-maximum/</a></p>

<p>&nbsp;</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/hua-dong-chuang-kou-de-zui-da-zhi-lcof/
// discuss: https://leetcode.com/problems/hua-dong-chuang-kou-de-zui-da-zhi-lcof/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn max_altitude(heights: Vec<i32>, limit: i32) -> Vec<i32> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCR 183() {
    }
}