# circular_matrix

> 输出任意阶类似下列蛇形矩阵<br />
<code> 1 2&nbsp;&nbsp;&nbsp;1 2 3&nbsp;&nbsp;&nbsp;&nbsp;1 &nbsp;2 &nbsp;3 &nbsp;4<br />
4 3&nbsp;&nbsp;&nbsp;8 9 4&nbsp;&nbsp;&nbsp;12 13 14 &nbsp;5<br />
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;7 6 5&nbsp;&nbsp;&nbsp;11 16 15&nbsp; 6<br />
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;10  &nbsp;9  &nbsp;8&nbsp; 7
</code>

# get_num.py

> 存在一个 n 位正整数 A 可写作：<br />
<code>a<sub>n</sub>a<sub>n-1</sub>a<sub>n-2</sub>...a<sub>1</sub></code><br />
经过如下位置调换得到正整数 A' 可写作：<br />
<code>a<sub>1</sub>a<sub>n</sub>a<sub>n-1</sub>a<sub>n-2</sub>...a<sub>2</sub></code><br />
A 与 A' 存在如下关系：<br />
<code> A' = 2A</code>

## solution

>设辅助数列记为 {k<sub>n</sub>}, 假设正整数 A 的个位数为k<sub>n+1</sub>存在如下关系:<br />
<code>
k<sub>n</sub> = k<sub>n+1</sub>%2*10 + int(k<sub>n+1</sub>/2)<br />
k<sub>n-1</sub> = k<sub>n</sub>%2*10 + int(k<sub>n</sub>/2)<br />
k<sub>n-2</sub> = k<sub>n-1</sub>%2*10 + int(k<sub>n-1</sub>/2)<br />
...
</code><br />
则对于数列{a<sub>n</sub>},有<br />
<code>
a<sub>n</sub> = int(k<sub>n+1</sub>/2)<br />
a<sub>n-1</sub> = int(k<sub>n</sub>/2)<br />
...
</code><br />
对于数列{k<sub>n</sub>}的递归中止条件为：<br />
<code>
k<sub>i</sub>/2 = k<sub>n+1</sub>
</code>

