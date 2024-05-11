/**
 * [P001353] Maximum Number of Events That Can Be Attended
 *
 * <p>给你一个数组&nbsp;<code>events</code>，其中&nbsp;<code>events[i] = [startDay<sub>i</sub>, endDay<sub>i</sub>]</code>&nbsp;，表示会议&nbsp;<code>i</code>&nbsp;开始于&nbsp;<code>startDay<sub>i</sub></code>&nbsp;，结束于&nbsp;<code>endDay<sub>i</sub></code>&nbsp;。</p>

<p>你可以在满足&nbsp;<code>startDay<sub>i</sub>&nbsp;&lt;= d &lt;= endDay<sub>i</sub></code><sub>&nbsp;</sub>中的任意一天&nbsp;<code>d</code>&nbsp;参加会议&nbsp;<code>i</code>&nbsp;。注意，一天只能参加一个会议。</p>

<p>请你返回你可以参加的&nbsp;<strong>最大&nbsp;</strong>会议数目。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<p><img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/02/16/e1.png" style="height: 267px; width: 400px;" /></p>

<pre>
<strong>输入：</strong>events = [[1,2],[2,3],[3,4]]
<strong>输出：</strong>3
<strong>解释：</strong>你可以参加所有的三个会议。
安排会议的一种方案如上图。
第 1 天参加第一个会议。
第 2 天参加第二个会议。
第 3 天参加第三个会议。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>events= [[1,2],[2,3],[3,4],[1,2]]
<strong>输出：</strong>4
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong>​​​​​​</p>

<ul>
	<li><code>1 &lt;= events.length &lt;= 10<sup>5</sup></code></li>
	<li><code>events[i].length == 2</code></li>
	<li><code>1 &lt;= startDay<sub>i</sub>&nbsp;&lt;= endDay<sub>i</sub>&nbsp;&lt;= 10<sup>5</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/
// discuss: https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001353() {
    }
}