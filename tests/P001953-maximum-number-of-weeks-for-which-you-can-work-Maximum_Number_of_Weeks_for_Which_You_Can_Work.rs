/**
 * [P001953] Maximum Number of Weeks for Which You Can Work
 *
 * <p>给你&nbsp;<code>n</code> 个项目，编号从 <code>0</code> 到 <code>n - 1</code> 。同时给你一个整数数组 <code>milestones</code> ，其中每个 <code>milestones[i]</code> 表示第 <code>i</code> 个项目中的阶段任务数量。</p>

<p>你可以按下面两个规则参与项目中的工作：</p>

<ul>
	<li>每周，你将会完成 <strong>某一个</strong> 项目中的 <strong>恰好一个</strong>&nbsp;阶段任务。你每周都 <strong>必须</strong> 工作。</li>
	<li>在 <strong>连续的</strong> 两周中，你 <strong>不能</strong> 参与并完成同一个项目中的两个阶段任务。</li>
</ul>

<p>一旦所有项目中的全部阶段任务都完成，或者仅剩余一个阶段任务都会导致你违反上面的规则，那么你将&nbsp;<strong>停止工作</strong> 。注意，由于这些条件的限制，你可能无法完成所有阶段任务。</p>

<p>返回在不违反上面规则的情况下你&nbsp;<strong>最多</strong>&nbsp;能工作多少周。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>milestones = [1,2,3]
<strong>输出：</strong>6
<strong>解释：</strong>一种可能的情形是：
​​​​- 第 1 周，你参与并完成项目 0 中的一个阶段任务。
- 第 2 周，你参与并完成项目 2 中的一个阶段任务。
- 第 3 周，你参与并完成项目 1 中的一个阶段任务。
- 第 4 周，你参与并完成项目 2 中的一个阶段任务。
- 第 5 周，你参与并完成项目 1 中的一个阶段任务。
- 第 6 周，你参与并完成项目 2 中的一个阶段任务。
总周数是 6 。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>milestones = [5,2,1]
<strong>输出：</strong>7
<strong>解释：</strong>一种可能的情形是：
- 第 1 周，你参与并完成项目 0 中的一个阶段任务。
- 第 2 周，你参与并完成项目 1 中的一个阶段任务。
- 第 3 周，你参与并完成项目 0 中的一个阶段任务。
- 第 4 周，你参与并完成项目 1 中的一个阶段任务。
- 第 5 周，你参与并完成项目 0 中的一个阶段任务。
- 第 6 周，你参与并完成项目 2 中的一个阶段任务。
- 第 7 周，你参与并完成项目 0 中的一个阶段任务。
总周数是 7 。
注意，你不能在第 8 周参与完成项目 0 中的最后一个阶段任务，因为这会违反规则。
因此，项目 0 中会有一个阶段任务维持未完成状态。</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>n == milestones.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= milestones[i] &lt;= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/maximum-number-of-weeks-for-which-you-can-work/
// discuss: https://leetcode.com/problems/maximum-number-of-weeks-for-which-you-can-work/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001953() {
    }
}