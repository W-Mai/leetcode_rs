/**
 * [P000858] Mirror Reflection
 *
 * <p>有一个特殊的正方形房间，每面墙上都有一面镜子。除西南角以外，每个角落都放有一个接受器，编号为&nbsp;<code>0</code>，&nbsp;<code>1</code>，以及&nbsp;<code>2</code>。</p>

<p>正方形房间的墙壁长度为&nbsp;<code>p</code>，一束激光从西南角射出，首先会与东墙相遇，入射点到接收器 <code>0</code> 的距离为 <code>q</code> 。</p>

<p>返回光线最先遇到的接收器的编号（保证光线最终会遇到一个接收器）。</p>
&nbsp;

<p><strong class="example">示例 1：</strong></p>
<img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/06/18/reflection.png" style="width: 218px; height: 217px;" />
<pre>
<strong>输入：</strong>p = 2, q = 1
<strong>输出：</strong>2
<strong>解释：</strong>这条光线在第一次被反射回左边的墙时就遇到了接收器 2 。
</pre>

<p><strong class="example">示例 2：</strong></p>

<pre>
<strong>输入：</strong>p = 3, q = 1
<strong>输入：</strong>1
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= q &lt;= p &lt;= 1000</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/mirror-reflection/
// discuss: https://leetcode.com/problems/mirror-reflection/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000858() {
    }
}