/**
 * [P000787] Cheapest Flights Within K Stops
 *
 * <p>有 <code>n</code> 个城市通过一些航班连接。给你一个数组&nbsp;<code>flights</code> ，其中&nbsp;<code>flights[i] = [from<sub>i</sub>, to<sub>i</sub>, price<sub>i</sub>]</code> ，表示该航班都从城市 <code>from<sub>i</sub></code> 开始，以价格 <code>price<sub>i</sub></code> 抵达 <code>to<sub>i</sub></code>。</p>

<p>现在给定所有的城市和航班，以及出发城市 <code>src</code> 和目的地 <code>dst</code>，你的任务是找到出一条最多经过 <code>k</code>&nbsp;站中转的路线，使得从 <code>src</code> 到 <code>dst</code> 的 <strong>价格最便宜</strong> ，并返回该价格。 如果不存在这样的路线，则输出 <code>-1</code>。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入:</strong> 
n = 3, edges = [[0,1,100],[1,2,100],[0,2,500]]
src = 0, dst = 2, k = 1
<strong>输出:</strong> 200
<strong>解释:</strong> 
城市航班图如下
<img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/02/16/995.png" style="height: 180px; width: 246px;" />

从城市 0 到城市 2 在 1 站中转以内的最便宜价格是 200，如图中红色所示。</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入:</strong> 
n = 3, edges = [[0,1,100],[1,2,100],[0,2,500]]
src = 0, dst = 2, k = 0
<strong>输出:</strong> 500
<strong>解释:</strong> 
城市航班图如下
<img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/02/16/995.png" style="height: 180px; width: 246px;" />

从城市 0 到城市 2 在 0 站中转以内的最便宜价格是 500，如图中蓝色所示。</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 100</code></li>
	<li><code>0 &lt;= flights.length &lt;= (n * (n - 1) / 2)</code></li>
	<li><code>flights[i].length == 3</code></li>
	<li><code>0 &lt;= from<sub>i</sub>, to<sub>i</sub> &lt; n</code></li>
	<li><code>from<sub>i</sub> != to<sub>i</sub></code></li>
	<li><code>1 &lt;= price<sub>i</sub> &lt;= 10<sup>4</sup></code></li>
	<li>航班没有重复，且不存在自环</li>
	<li><code>0 &lt;= src, dst, k &lt; n</code></li>
	<li><code>src != dst</code></li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/cheapest-flights-within-k-stops/
// discuss: https://leetcode.com/problems/cheapest-flights-within-k-stops/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000787() {
    }
}