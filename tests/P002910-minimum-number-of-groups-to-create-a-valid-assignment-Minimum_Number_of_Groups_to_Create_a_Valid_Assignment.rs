/**
 * [P002910] Minimum Number of Groups to Create a Valid Assignment
 *
 * <p>给你一组带编号的&nbsp;<code>balls</code> 并要求将它们分类到盒子里，以便均衡地分配。你必须遵守两条规则：</p>

<ul>
	<li>同一个盒子里的球必须具有相同的编号。但是，如果你有多个相同编号的球，你可以把它们放在不同的盒子里。</li>
	<li>最大的盒子只能比最小的盒子多一个球。</li>
</ul>

<p>返回遵循上述规则排列这些球所需要的盒子的最小数目。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<pre>
<b>输入：</b>balls = [3,2,3,2,3]
<b>输出：</b>2
<b>解释：</b>一个得到 2 个分组的方案如下，中括号内的数字都是下标：
我们可以如下排列 balls 到盒子里：
- [3,3,3]
- [2,2]
两个盒子之间的大小差没有超过 1。</pre>

<p><strong class="example">示例 2：</strong></p>

<pre>
<b>输入：</b>balls = [10,10,10,3,1,1]
<b>输出：</b>4
<b>解释：</b>我们可以如下排列 balls 到盒子里：
- [10]
- [10,10]
- [3]
- [1,1]
无法得到一个遵循上述规则且小于 4 盒的答案。例如，把所有三个编号为 10 的球都放在一个盒子里，就会打破盒子之间最大尺寸差异的规则。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= balls.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= balls[i] &lt;= 10<sup>9</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/minimum-number-of-groups-to-create-a-valid-assignment/
// discuss: https://leetcode.com/problems/minimum-number-of-groups-to-create-a-valid-assignment/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn min_groups_for_valid_assignment(balls: Vec<i32>) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002910() {
    }
}