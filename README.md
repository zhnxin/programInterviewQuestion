# circular_matrix

> 输出任意阶类似下列蛇形矩阵<br />
<code> 1 2&nbsp;&nbsp;&nbsp;1 2 3&nbsp;&nbsp;&nbsp;&nbsp;1 &nbsp;2 &nbsp;3 &nbsp;4</code><br/>
<code>4 3&nbsp;&nbsp;&nbsp;8 9 4&nbsp;&nbsp;&nbsp;12 13 14 &nbsp;5</code><br/>
<code>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;7 6 5&nbsp;&nbsp;&nbsp;11 16 15&nbsp; 6</code><br/>
<code>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;10  &nbsp;9  &nbsp;8&nbsp; 7</code>

# get_num.py

> 存在一个 n 位正整数 A 可写作：<br />
<code>a<sub>n</sub>a<sub>n-1</sub>a<sub>n-2</sub>...a<sub>1</sub></code><br />
经过如下位置调换得到正整数 A' 可写作：<br />
<code>a<sub>1</sub>a<sub>n</sub>a<sub>n-1</sub>a<sub>n-2</sub>...a<sub>2</sub></code><br />
A 与 A' 存在如下关系：<br />
<code> A' = 2A</code>

### note

>设辅助数列记为 {k<sub>n</sub>}, 假设正整数 A 的个位数为k<sub>n+1</sub>存在如下关系:<br />
<code>k<sub>n</sub> = 10 * k<sub>n+1</sub>%2 + int(k<sub>n+1</sub>/2)</code><br />
<code>k<sub>n-1</sub> = 10 * k<sub>n</sub>%2 + int(k<sub>n</sub>/2)</code><br />
<code>k<sub>n-2</sub> = 10 * k<sub>n-1</sub>%2 + int(k<sub>n-1</sub>/2)</code><br />
<code>...</code><br />
则对于数列{a<sub>n</sub>},有<br />
<code>a<sub>n</sub> = int(k<sub>n+1</sub>/2)</code><br />
<code>a<sub>n-1</sub> = int(k<sub>n</sub>/2)</code><br />
<code>...</code><br />
对于数列{k<sub>n</sub>}的递归中止条件为：<br />
<code>k<sub>i</sub>/2 = k<sub>n+1</sub></code>

#bubblesort.cpp

>使用迭代而非循环实现冒泡排序

### note
>添加一个标识符——flag，初始化为true，当一次遍历过程中存在交换时置为false。<br/>
迭代中止条件为遍历到list尾部且flag为true，即本次遍历不存在交换，否则重置flag，并从新开始遍历。


