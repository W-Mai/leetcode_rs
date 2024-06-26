/**
 * [P001759] Count Number of Homogenous Substrings
 *
 * <p>给你一个字符串 <code>s</code> ，返回<em> </em><code>s</code><em> </em>中 <strong>同质子字符串</strong> 的数目。由于答案可能很大，只需返回对 <code>10<sup>9</sup> + 7</code> <strong>取余 </strong>后的结果。</p>

<p><strong>同质字符串</strong> 的定义为：如果一个字符串中的所有字符都相同，那么该字符串就是同质字符串。</p>

<p><strong>子字符串</strong> 是字符串中的一个连续字符序列。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>s = "abbcccaa"
<strong>输出：</strong>13
<strong>解释：</strong>同质子字符串如下所列：
"a"   出现 3 次。
"aa"  出现 1 次。
"b"   出现 2 次。
"bb"  出现 1 次。
"c"   出现 3 次。
"cc"  出现 2 次。
"ccc" 出现 1 次。
3 + 1 + 2 + 1 + 3 + 2 + 1 = 13</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>s = "xy"
<strong>输出：</strong>2
<strong>解释：</strong>同质子字符串是 "x" 和 "y" 。</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>s = "zzzzz"
<strong>输出：</strong>15
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> 由小写字符串组成。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/count-number-of-homogenous-substrings/
// discuss: https://leetcode.com/problems/count-number-of-homogenous-substrings/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001759() {
    }
}