rust没有语义保证的tail call optimization(TCO,尾递归)。Box的drop实现不是尾递归。

相关代码见learn_linkedList_problem.rs