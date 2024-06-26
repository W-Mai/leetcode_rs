/**
 * [P002998] Minimum Number of Operations to Make X and Y Equal
 *
 * <p>给你两个正整数&nbsp;<code>x</code> 和&nbsp;<code>y</code>&nbsp;。</p>

<p>一次操作中，你可以执行以下四种操作之一：</p>

<ol>
	<li>如果 <code>x</code>&nbsp;是 <code>11</code>&nbsp;的倍数，将&nbsp;<code>x</code>&nbsp;除以&nbsp;<code>11</code>&nbsp;。</li>
	<li>如果 <code>x</code>&nbsp;是 <code>5</code>&nbsp;的倍数，将 <code>x</code>&nbsp;除以 <code>5</code>&nbsp;。</li>
	<li>将&nbsp;<code>x</code> 减&nbsp;<code>1</code>&nbsp;。</li>
	<li>将&nbsp;<code>x</code>&nbsp;加&nbsp;<code>1</code>&nbsp;。</li>
</ol>

<p>请你返回让 <code>x</code>&nbsp;和 <code>y</code>&nbsp;相等的 <strong>最少</strong>&nbsp;操作次数。</p>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<pre>
<b>输入：</b>x = 26, y = 1
<b>输出：</b>3
<b>解释</b><strong>：</strong>我们可以通过以下操作将 26 变为 1 ：
1. 将 x 减 1
2. 将 x 除以 5
3. 将 x 除以 5
将 26 变为 1 最少需要 3 次操作。
</pre>

<p><strong class="example">示例 2：</strong></p>

<pre>
<b>输入：</b>x = 54, y = 2
<b>输出：</b>4
<b>解释：</b>我们可以通过以下操作将 54 变为 2 ：
1. 将 x 加 1
2. 将 x 除以 11
3. 将 x 除以 5
4. 将 x 加 1
将 54 变为 2 最少需要 4 次操作。
</pre>

<p><strong class="example">示例 3：</strong></p>

<pre>
<b>输入：</b>x = 25, y = 30
<b>输出：</b>5
<b>解释：</b>我们可以通过以下操作将 25 变为 30 ：
1. 将 x 加 1
2. 将 x 加 1
3. 将 x 加 1
4. 将 x 加 1
5. 将 x 加 1
将 25 变为 30 最少需要 5 次操作。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= x, y &lt;= 10<sup>4</sup></code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/minimum-number-of-operations-to-make-x-and-y-equal/
// discuss: https://leetcode.com/problems/minimum-number-of-operations-to-make-x-and-y-equal/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn minimum_operations_to_make_equal(x: i32, y: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002998() {
    }
}