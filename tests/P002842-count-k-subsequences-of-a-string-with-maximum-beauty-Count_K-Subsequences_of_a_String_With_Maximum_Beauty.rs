/**
 * [P002842] Count K-Subsequences of a String With Maximum Beauty
 *
 * <p>给你一个字符串&nbsp;<code>s</code>&nbsp;和一个整数&nbsp;<code>k</code>&nbsp;。</p>

<p><strong>k 子序列</strong>指的是 <code>s</code>&nbsp;的一个长度为 <code>k</code>&nbsp;的 <strong>子序列</strong>&nbsp;，且所有字符都是 <strong>唯一</strong>&nbsp;的，也就是说每个字符在子序列里只出现过一次。</p>

<p>定义&nbsp;<code>f(c)</code>&nbsp;为字符 <code>c</code>&nbsp;在 <code>s</code>&nbsp;中出现的次数。</p>

<p>k 子序列的 <strong>美丽值</strong>&nbsp;定义为这个子序列中每一个字符 <code>c</code>&nbsp;的&nbsp;<code>f(c)</code>&nbsp;之 <strong>和</strong>&nbsp;。</p>

<p>比方说，<code>s = "abbbdd"</code>&nbsp;和&nbsp;<code>k = 2</code>&nbsp;，我们有：</p>

<ul>
	<li><code>f('a') = 1</code>, <code>f('b') = 3</code>, <code>f('d') = 2</code></li>
	<li><code>s</code>&nbsp;的部分 k 子序列为：
	<ul>
		<li><code>"<em><strong>ab</strong></em>bbdd"</code> -&gt; <code>"ab"</code>&nbsp;，美丽值为&nbsp;<code>f('a') + f('b') = 4</code></li>
		<li><code>"<em><strong>a</strong></em>bbb<em><strong>d</strong></em>d"</code> -&gt; <code>"ad"</code>&nbsp;，美丽值为&nbsp;<code>f('a') + f('d') = 3</code></li>
		<li><code>"a<em><strong>b</strong></em>bb<em><strong>d</strong></em>d"</code> -&gt; <code>"bd"</code>&nbsp;，美丽值为&nbsp;<code>f('b') + f('d') = 5</code></li>
	</ul>
	</li>
</ul>

<p>请你返回一个整数，表示所有 <strong>k 子序列&nbsp;</strong>里面 <strong>美丽值 </strong>是&nbsp;<strong>最大值</strong>&nbsp;的子序列数目。由于答案可能很大，将结果对&nbsp;<code>10<sup>9</sup> + 7</code>&nbsp;取余后返回。</p>

<p>一个字符串的子序列指的是从原字符串里面删除一些字符（也可能一个字符也不删除），不改变剩下字符顺序连接得到的新字符串。</p>

<p><strong>注意：</strong></p>

<ul>
	<li><code>f(c)</code> 指的是字符&nbsp;<code>c</code>&nbsp;在字符串&nbsp;<code>s</code>&nbsp;的出现次数，不是在 k 子序列里的出现次数。</li>
	<li>两个 k 子序列如果有任何一个字符在原字符串中的下标不同，则它们是两个不同的子序列。所以两个不同的 k 子序列可能产生相同的字符串。</li>
</ul>

<p>&nbsp;</p>

<p><strong class="example">示例 1：</strong></p>

<pre>
<b>输入：</b>s = "bcca", k = 2
<b>输出：</b>4
<b>解释：</b><span style="white-space: normal">s 中我们有 f('a') = 1 ，f('b') = 1 和 f('c') = 2 。</span>
s 的 k 子序列为：
<em><strong>bc</strong></em>ca ，美丽值为 f('b') + f('c') = 3
<em><strong>b</strong></em>c<em><strong>c</strong></em>a ，美丽值为 f('b') + f('c') = 3
<em><strong>b</strong></em>cc<em><strong>a</strong></em> ，美丽值为 f('b') + f('a') = 2
b<em><strong>c</strong></em>c<em><strong>a</strong></em><strong> </strong>，美丽值为 f('c') + f('a') = 3
bc<em><strong>ca</strong></em> ，美丽值为 f('c') + f('a') = 3
总共有 4 个 k 子序列美丽值为最大值 3 。
所以答案为 4 。
</pre>

<p><strong class="example">示例 2：</strong></p>

<pre>
<b>输入：</b>s = "abbcd", k = 4
<b>输出：</b>2
<b>解释：</b><span style="white-space: normal">s 中我们有 f('a') = 1 ，f('b') = 2&nbsp;，f('c') = 1&nbsp;和</span> f('d') = 1 。
s 的 k 子序列为：
<em><strong>ab</strong></em>b<em><strong>cd</strong></em> ，美丽值为 f('a') + f('b') + f('c') + f('d') = 5
<span style="white-space: normal;"><b><i>a</i></b></span>b<em><strong>bcd</strong></em> ，美丽值为 f('a') + f('b') + f('c') + f('d') = 5 
总共有 2 个 k 子序列美丽值为最大值 5 。
所以答案为 2 。
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 2 * 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= s.length</code></li>
	<li><code>s</code>&nbsp;只包含小写英文字母。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/count-k-subsequences-of-a-string-with-maximum-beauty/
// discuss: https://leetcode.com/problems/count-k-subsequences-of-a-string-with-maximum-beauty/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn count_k_subsequences_with_max_beauty(s: String, k: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P002842() {
    }
}