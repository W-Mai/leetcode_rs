/**
 * [P000553] Optimal Division
 *
 * <p>给定一正整数数组<strong> </strong><code>nums</code><strong>，</strong><code>nums</code> 中的相邻整数将进行浮点除法。例如，&nbsp;[2,3,4] -&gt; 2 / 3 / 4 。</p>

<ul>
	<li>例如，<code>nums = [2,3,4]</code>，我们将求表达式的值&nbsp;<code>"2/3/4"</code>。</li>
</ul>

<p>但是，你可以在任意位置添加任意数目的括号，来改变算数的优先级。你需要找出怎么添加括号，以便计算后的表达式的值为最大值。</p>

<p>以字符串格式返回具有最大值的对应表达式。</p>

<p><strong>注意：</strong>你的表达式不应该包含多余的括号。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入:</strong> [1000,100,10,2]
<strong>输出:</strong> "1000/(100/10/2)"
<strong>解释: </strong>1000/(100/10/2) = 1000/((100/10)/2) = 200
但是，以下加粗的括号 "1000/(<strong>(</strong>100/10<strong>)</strong>/2)" 是冗余的，
因为他们并不影响操作的优先级，所以你需要返回 "1000/(100/10/2)"。

其他用例:
1000/(100/10)/2 = 50
1000/(100/(10/2)) = 50
1000/100/10/2 = 0.5
1000/100/(10/2) = 2
</pre>

<p>&nbsp;</p>

<p><strong>示例 2:</strong></p>

<pre>
<strong>输入:</strong> nums = [2,3,4]
<strong>输出:</strong> "2/(3/4)"
<strong>解释:</strong> (2/(3/4)) = 8/3 = 2.667
可以看出，在尝试了所有的可能性之后，我们无法得到一个结果大于 2.667 的表达式。
</pre>

<p>&nbsp;</p>

<p><strong>说明:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10</code></li>
	<li><code>2 &lt;= nums[i] &lt;= 1000</code></li>
	<li>对于给定的输入只有一种最优除法。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/optimal-division/
// discuss: https://leetcode.com/problems/optimal-division/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000553() {
    }
}