/**
 * [P000355] Design Twitter
 *
 * <p>设计一个简化版的推特(Twitter)，可以让用户实现发送推文，关注/取消关注其他用户，能够看见关注人（包括自己）的最近 <code>10</code> 条推文。</p>

<p>实现 <code>Twitter</code> 类：</p>

<ul>
	<li><code>Twitter()</code> 初始化简易版推特对象</li>
	<li><code>void postTweet(int userId, int tweetId)</code> 根据给定的 <code>tweetId</code> 和 <code>userId</code> 创建一条新推文。每次调用此函数都会使用一个不同的 <code>tweetId</code> 。</li>
	<li><code>List&lt;Integer&gt; getNewsFeed(int userId)</code> 检索当前用户新闻推送中最近&nbsp; <code>10</code> 条推文的 ID 。新闻推送中的每一项都必须是由用户关注的人或者是用户自己发布的推文。推文必须 <strong>按照时间顺序由最近到最远排序</strong> 。</li>
	<li><code>void follow(int followerId, int followeeId)</code> ID 为 <code>followerId</code> 的用户开始关注 ID 为 <code>followeeId</code> 的用户。</li>
	<li><code>void unfollow(int followerId, int followeeId)</code> ID 为 <code>followerId</code> 的用户不再关注 ID 为 <code>followeeId</code> 的用户。</li>
</ul>

<p>&nbsp;</p>

<p><strong>示例：</strong></p>

<pre>
<strong>输入</strong>
["Twitter", "postTweet", "getNewsFeed", "follow", "postTweet", "getNewsFeed", "unfollow", "getNewsFeed"]
[[], [1, 5], [1], [1, 2], [2, 6], [1], [1, 2], [1]]
<strong>输出</strong>
[null, null, [5], null, null, [6, 5], null, [5]]

<strong>解释</strong>
Twitter twitter = new Twitter();
twitter.postTweet(1, 5); // 用户 1 发送了一条新推文 (用户 id = 1, 推文 id = 5)
twitter.getNewsFeed(1);  // 用户 1 的获取推文应当返回一个列表，其中包含一个 id 为 5 的推文
twitter.follow(1, 2);    // 用户 1 关注了用户 2
twitter.postTweet(2, 6); // 用户 2 发送了一个新推文 (推文 id = 6)
twitter.getNewsFeed(1);  // 用户 1 的获取推文应当返回一个列表，其中包含两个推文，id 分别为 -&gt; [6, 5] 。推文 id 6 应当在推文 id 5 之前，因为它是在 5 之后发送的
twitter.unfollow(1, 2);  // 用户 1 取消关注了用户 2
twitter.getNewsFeed(1);  // 用户 1 获取推文应当返回一个列表，其中包含一个 id 为 5 的推文。因为用户 1 已经不再关注用户 2</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= userId, followerId, followeeId &lt;= 500</code></li>
	<li><code>0 &lt;= tweetId &lt;= 10<sup>4</sup></code></li>
	<li>所有推特的 ID 都互不相同</li>
	<li><code>postTweet</code>、<code>getNewsFeed</code>、<code>follow</code> 和 <code>unfollow</code> 方法最多调用 <code>3 * 10<sup>4</sup></code> 次</li>
</ul>

 */
pub struct Solution {}



// problem: https://leetcode.com/problems/design-twitter/
// discuss: https://leetcode.com/problems/design-twitter/discuss/

// >>> SUBMISSION CODES START HERE <<<

struct Twitter {

}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    fn new() -> Self {

    }
    
    fn post_tweet(&self, user_id: i32, tweet_id: i32) {

    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {

    }
    
    fn follow(&self, follower_id: i32, followee_id: i32) {

    }
    
    fn unfollow(&self, follower_id: i32, followee_id: i32) {

    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_P000355() {
    }
}