/**
 * [P002849] Determine if a Cell Is Reachable at a Given Time
 *
 * <p>给你四个整数 <code>sx</code>、<code>sy</code>、<code>fx</code>、<code>fy</code>&nbsp; 以及一个 <strong>非负整数</strong> <code>t</code> 。</p>

<p>在一个无限的二维网格中，你从单元格 <code>(sx, sy)</code> 开始出发。每一秒，你 <strong>必须</strong> 移动到任一与之前所处单元格相邻的单元格中。</p>

<p>如果你能在 <strong>恰好 </strong><code>t</code><strong> 秒</strong> 后到达单元格<em> </em><code>(fx, fy)</code> ，返回 <code>true</code> ；否则，返回&nbsp; <code>false</code> 。</p>

<p>单元格的 <strong>相邻单元格</strong> 是指该单元格周围与其至少共享一个角的 8 个单元格。你可以多次访问同一个单元格。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/08/05/example2.svg" style="width: 443px; height: 243px;" />
<pre>
<strong>输入：</strong>sx = 2, sy = 4, fx = 7, fy = 7, t = 6
<strong>输出：</strong>true
<strong>解释：</strong>从单元格 (2, 4) 开始出发，穿过上图标注的单元格，可以在恰好 6 秒后到达单元格 (7, 7) 。 
</pre>

<p><strong class="example">示例 2：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/08/05/example1.svg" style="width: 383px; height: 202px;" />
<pre>
<strong>输入：</strong>sx = 3, sy = 1, fx = 7, fy = 3, t = 3
<strong>输出：</strong>false
<strong>解释：</strong>从单元格 (3, 1) 开始出发，穿过上图标注的单元格，至少需要 4 秒后到达单元格 (7, 3) 。 因此，无法在 3 秒后到达单元格 (7, 3) 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= sx, sy, fx, fy &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= t &lt;= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/
// discuss: https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002849() {
    }
}