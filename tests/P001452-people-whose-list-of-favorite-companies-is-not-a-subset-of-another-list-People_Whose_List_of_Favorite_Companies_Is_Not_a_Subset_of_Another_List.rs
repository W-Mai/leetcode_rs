/**
 * [P001452] People Whose List of Favorite Companies Is Not a Subset of Another List
 *
 * <p>给你一个数组 <code>favoriteCompanies</code> ，其中 <code>favoriteCompanies[i]</code> 是第 <code>i</code> 名用户收藏的公司清单（<strong>下标从 0 开始</strong>）。</p>

<p>请找出不是其他任何人收藏的公司清单的子集的收藏清单，并返回该清单下标<em>。</em>下标需要按升序排列<em>。</em></p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>favoriteCompanies = [[&quot;leetcode&quot;,&quot;google&quot;,&quot;facebook&quot;],[&quot;google&quot;,&quot;microsoft&quot;],[&quot;google&quot;,&quot;facebook&quot;],[&quot;google&quot;],[&quot;amazon&quot;]]
<strong>输出：</strong>[0,1,4] 
<strong>解释：</strong>
favoriteCompanies[2]=[&quot;google&quot;,&quot;facebook&quot;] 是 favoriteCompanies[0]=[&quot;leetcode&quot;,&quot;google&quot;,&quot;facebook&quot;] 的子集。
favoriteCompanies[3]=[&quot;google&quot;] 是 favoriteCompanies[0]=[&quot;leetcode&quot;,&quot;google&quot;,&quot;facebook&quot;] 和 favoriteCompanies[1]=[&quot;google&quot;,&quot;microsoft&quot;] 的子集。
其余的收藏清单均不是其他任何人收藏的公司清单的子集，因此，答案为 [0,1,4] 。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>favoriteCompanies = [[&quot;leetcode&quot;,&quot;google&quot;,&quot;facebook&quot;],[&quot;leetcode&quot;,&quot;amazon&quot;],[&quot;facebook&quot;,&quot;google&quot;]]
<strong>输出：</strong>[0,1] 
<strong>解释：</strong>favoriteCompanies[2]=[&quot;facebook&quot;,&quot;google&quot;] 是 favoriteCompanies[0]=[&quot;leetcode&quot;,&quot;google&quot;,&quot;facebook&quot;] 的子集，因此，答案为 [0,1] 。
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>favoriteCompanies = [[&quot;leetcode&quot;],[&quot;google&quot;],[&quot;facebook&quot;],[&quot;amazon&quot;]]
<strong>输出：</strong>[0,1,2,3]
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;=&nbsp;favoriteCompanies.length &lt;= 100</code></li>
	<li><code>1 &lt;=&nbsp;favoriteCompanies[i].length &lt;= 500</code></li>
	<li><code>1 &lt;=&nbsp;favoriteCompanies[i][j].length &lt;= 20</code></li>
	<li><code>favoriteCompanies[i]</code> 中的所有字符串 <strong>各不相同</strong> 。</li>
	<li>用户收藏的公司清单也 <strong>各不相同</strong> ，也就是说，即便我们按字母顺序排序每个清单， <code>favoriteCompanies[i] != favoriteCompanies[j] </code>仍然成立。</li>
	<li>所有字符串仅包含小写英文字母。</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/people-whose-list-of-favorite-companies-is-not-a-subset-of-another-list/
// discuss: https://leetcode.com/problems/people-whose-list-of-favorite-companies-is-not-a-subset-of-another-list/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P001452() {
    }
}