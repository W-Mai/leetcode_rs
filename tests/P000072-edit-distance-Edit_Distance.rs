/**
 * [P000072] Edit Distance
 *
 * <p>给你两个单词&nbsp;<code>word1</code> 和&nbsp;<code>word2</code>， <em>请返回将&nbsp;<code>word1</code>&nbsp;转换成&nbsp;<code>word2</code> 所使用的最少操作数</em> &nbsp;。</p>

<p>你可以对一个单词进行如下三种操作：</p>

<ul>
	<li>插入一个字符</li>
	<li>删除一个字符</li>
	<li>替换一个字符</li>
</ul>

<p>&nbsp;</p>

<p><strong>示例&nbsp;1：</strong></p>

<pre>
<strong>输入：</strong>word1 = "horse", word2 = "ros"
<strong>输出：</strong>3
<strong>解释：</strong>
horse -&gt; rorse (将 'h' 替换为 'r')
rorse -&gt; rose (删除 'r')
rose -&gt; ros (删除 'e')
</pre>

<p><strong>示例&nbsp;2：</strong></p>

<pre>
<strong>输入：</strong>word1 = "intention", word2 = "execution"
<strong>输出：</strong>5
<strong>解释：</strong>
intention -&gt; inention (删除 't')
inention -&gt; enention (将 'i' 替换为 'e')
enention -&gt; exention (将 'n' 替换为 'x')
exention -&gt; exection (将 'n' 替换为 'c')
exection -&gt; execution (插入 'u')
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>0 &lt;= word1.length, word2.length &lt;= 500</code></li>
	<li><code>word1</code> 和 <code>word2</code> 由小写英文字母组成</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/edit-distance/
// discuss: https://leetcode.com/problems/edit-distance/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000072() {
    }
}