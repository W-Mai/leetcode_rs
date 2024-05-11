/**
 * [LCR 129] 字母迷宫
 *
 * <p>字母迷宫游戏初始界面记作 <code>m x n</code> 二维字符串数组 <code>grid</code>，请判断玩家是否能在 <code>grid</code> 中找到目标单词 <code>target</code>。<br />
注意：寻找单词时 <strong>必须</strong> 按照字母顺序，通过水平或垂直方向相邻的单元格内的字母构成，同时，同一个单元格内的字母&nbsp;<strong>不允许被重复使用&nbsp;</strong>。</p>

<p>&nbsp;</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/word2.jpg" /></p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>grid = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], target = "ABCCED"
<strong>输出：</strong>true
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>grid = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], target = "SEE"
<strong>输出：</strong>true
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>grid = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], target = "ABCB"
<strong>输出：</strong>false
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n = grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 6</code></li>
	<li><code>1 &lt;= target.length &lt;= 15</code></li>
	<li><code>grid</code> 和 <code>target</code> 仅由大小写英文字母组成</li>
</ul>

<p>&nbsp;</p>

<p><strong>注意：</strong>本题与主站 79 题相同：<a href="https://leetcode-cn.com/problems/word-search/">https://leetcode-cn.com/problems/word-search/</a></p>

<p>&nbsp;</p>

<p>&nbsp;</p>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/ju-zhen-zhong-de-lu-jing-lcof/
// discuss: https://leetcode.com/problems/ju-zhen-zhong-de-lu-jing-lcof/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn word_puzzle(grid: Vec<Vec<char>>, target: String) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCR 129() {
    }
}