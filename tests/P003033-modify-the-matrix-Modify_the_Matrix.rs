/**
 * [P003033] Modify the Matrix
 *
 * <p>给你一个下标从 <strong>0</strong> 开始、大小为 <code>m x n</code> 的整数矩阵 <code>matrix</code> ，新建一个下标从 <strong>0</strong> 开始、名为 <code>answer</code> 的矩阵。使 <code>answer</code> 与 <code>matrix</code> 相等，接着将其中每个值为 <code>-1</code> 的元素替换为所在列的 <strong>最大</strong> 元素。</p>

<p>返回矩阵 <code>answer</code> 。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/12/24/matrix1.png" style="width: 491px; height: 161px;" />
<pre>
<strong>输入：</strong>matrix = [[1,2,-1],[4,-1,6],[7,8,9]]
<strong>输出：</strong>[[1,2,9],[4,8,6],[7,8,9]]
<strong>解释：</strong>上图显示了发生替换的元素（蓝色区域）。
- 将单元格 [1][1] 中的值替换为列 1 中的最大值 8 。
- 将单元格 [0][2] 中的值替换为列 2 中的最大值 9 。
</pre>

<p><strong class="example">示例 2：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/12/24/matrix2.png" style="width: 411px; height: 111px;" />
<pre>
<strong>输入：</strong>matrix = [[3,-1],[5,2]]
<strong>输出：</strong>[[3,2],[5,2]]
<strong>解释：</strong>上图显示了发生替换的元素（蓝色区域）。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>m == matrix.length</code></li>
	<li><code>n == matrix[i].length</code></li>
	<li><code>2 &lt;= m, n &lt;= 50</code></li>
	<li><code>-1 &lt;= matrix[i][j] &lt;= 100</code></li>
	<li>测试用例中生成的输入满足每列至少包含一个非负整数。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/modify-the-matrix/
// discuss: https://leetcode.com/problems/modify-the-matrix/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P003033() {
    }
}