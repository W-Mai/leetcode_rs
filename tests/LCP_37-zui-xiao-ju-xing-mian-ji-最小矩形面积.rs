/**
 * [LCP 37] 最小矩形面积
 *
 * 二维平面上有 $N$ 条直线，形式为 `y = kx + b`，其中 `k`、`b`为整数 且 `k > 0`。所有直线以 `[k,b]` 的形式存于二维数组 `lines` 中，不存在重合的两条直线。两两直线之间可能存在一个交点，最多会有 $C_N^2$ 个交点。我们用一个平行于坐标轴的矩形覆盖所有的交点，请问这个矩形最小面积是多少。若直线之间无交点、仅有一个交点或所有交点均在同一条平行坐标轴的直线上，则返回0。

注意：返回结果是浮点数，与标准答案 **绝对误差或相对误差** 在 10^-4 以内的结果都被视为正确结果


**示例 1：**
> 输入：`lines = [[2,3],[3,0],[4,1]]`
>
> 输出：`48.00000`
>
> 解释：三条直线的三个交点为 (3, 9) (1, 5) 和 (-1, -3)。最小覆盖矩形左下角为 (-1, -3) 右上角为 (3,9)，面积为 48


**示例 2：**
> 输入：`lines = [[1,1],[2,3]]`
>
> 输出：`0.00000`
>
> 解释：仅有一个交点 (-2，-1）


**限制：**
+ `1 <= lines.length <= 10^5 且 lines[i].length == 2`
+ `1 <= lines[0] <= 10000`
+ `-10000 <= lines[1] <= 10000`
+ `与标准答案绝对误差或相对误差在 10^-4 以内的结果都被视为正确结果`
 */
pub struct Solution {}



// problem: https://leetcode.com/problems/zui-xiao-ju-xing-mian-ji/
// discuss: https://leetcode.com/problems/zui-xiao-ju-xing-mian-ji/discuss/

// >>> SUBMISSION CODES START HERE <<<

impl Solution {
    pub fn min_rec_size(lines: Vec<Vec<i32>>) -> f64 {

    }
}

// >>>    SUBMISSION CODES END     <<<

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LCP 37() {
    }
}