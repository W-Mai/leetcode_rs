/**
 * [P000278] First Bad Version
 *
 * <p>你是产品经理，目前正在带领一个团队开发新的产品。不幸的是，你的产品的最新版本没有通过质量检测。由于每个版本都是基于之前的版本开发的，所以错误的版本之后的所有版本都是错的。</p>

<p>假设你有 <code>n</code> 个版本 <code>[1, 2, ..., n]</code>，你想找出导致之后所有版本出错的第一个错误的版本。</p>

<p>你可以通过调用 <code>bool isBadVersion(version)</code> 接口来判断版本号 <code>version</code> 是否在单元测试中出错。实现一个函数来查找第一个错误的版本。你应该尽量减少对调用 API 的次数。</p>
 

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>n = 5, bad = 4
<strong>输出：</strong>4
<strong>解释：</strong>
<code>调用 isBadVersion(3) -> false 
调用 isBadVersion(5) -> true 
调用 isBadVersion(4) -> true</code>
<code>所以，4 是第一个错误的版本。</code>
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>n = 1, bad = 1
<strong>输出：</strong>1
</pre>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 <= bad <= n <= 2<sup>31</sup> - 1</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/first-bad-version/
// discuss: https://leetcode.com/problems/first-bad-version/discuss/

// >>> SUBMISSION CODES START HERE <<<

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
		
    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000278() {
    }
}