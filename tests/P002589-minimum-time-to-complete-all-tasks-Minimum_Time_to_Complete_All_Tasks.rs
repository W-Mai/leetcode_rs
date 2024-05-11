/**
 * [P002589] Minimum Time to Complete All Tasks
 *
 * <p>你有一台电脑，它可以 <strong>同时</strong>&nbsp;运行无数个任务。给你一个二维整数数组&nbsp;<code>tasks</code>&nbsp;，其中&nbsp;<code>tasks[i] = [start<sub>i</sub>, end<sub>i</sub>, duration<sub>i</sub>]</code>&nbsp;表示第&nbsp;<code>i</code>&nbsp;个任务需要在 <strong>闭区间</strong>&nbsp;时间段&nbsp;<code>[start<sub>i</sub>, end<sub>i</sub>]</code>&nbsp;内运行&nbsp;<code>duration<sub>i</sub></code>&nbsp;个整数时间点（但不需要连续）。</p>

<p>当电脑需要运行任务时，你可以打开电脑，如果空闲时，你可以将电脑关闭。</p>

<p>请你返回完成所有任务的情况下，电脑最少需要运行多少秒。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><b>输入：</b>tasks = [[2,3,1],[4,5,1],[1,5,2]]
<b>输出：</b>2
<b>解释：</b>
- 第一个任务在闭区间 [2, 2] 运行。
- 第二个任务在闭区间 [5, 5] 运行。
- 第三个任务在闭区间 [2, 2] 和 [5, 5] 运行。
电脑总共运行 2 个整数时间点。
</pre>

<p><strong>示例 2：</strong></p>

<pre><b>输入：</b>tasks = [[1,3,2],[2,5,3],[5,6,2]]
<b>输出：</b>4
<b>解释：</b>
- 第一个任务在闭区间 [2, 3] 运行
- 第二个任务在闭区间 [2, 3] 和 [5, 5] 运行。
- 第三个任务在闭区间 [5, 6] 运行。
电脑总共运行 4 个整数时间点。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= tasks.length &lt;= 2000</code></li>
	<li><code>tasks[i].length == 3</code></li>
	<li><code>1 &lt;= start<sub>i</sub>, end<sub>i</sub> &lt;= 2000</code></li>
	<li><code>1 &lt;= duration<sub>i</sub> &lt;= end<sub>i</sub> - start<sub>i</sub> + 1 </code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/minimum-time-to-complete-all-tasks/
// discuss: https://leetcode.com/problems/minimum-time-to-complete-all-tasks/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn find_minimum_time(tasks: Vec<Vec<i32>>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002589() {
    }
}