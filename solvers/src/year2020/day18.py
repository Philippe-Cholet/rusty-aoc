'''
1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
'''

import ast
from functools import partial
from time import perf_counter as time

import pyperclip  # cross-platform clipboard (only handles plain text)


class SubToMult(ast.NodeTransformer):
    def visit_Sub(self, _node): return ast.Mult()


class SwapAddMult(ast.NodeTransformer):
    def visit_Mult(self, _node): return ast.Add()
    def visit_Add(self, _node): return ast.Mult()


def job(src_ops, dst_ops, transformer_cls, s):
    s = s.translate(str.maketrans(src_ops, dst_ops))
    node = ast.parse(s, mode='eval')
    transformer_cls().visit(node)
    return eval(compile(node, '', 'eval'))


# Because + and - have the same precedence, so the order is then left to right.
v1 = partial(job, '*', '-', SubToMult)
# Swap their precedences.
v2 = partial(job, '+*', '*+', SwapAddMult)
# Then in both cases, get the evaluation node, switch back operators then evaluate.

lines = pyperclip.paste().splitlines()
for func in v1, v2:
    if 0:
        print(*map(func, lines), sep='\n')
    else:
        t = -time()
        total = sum(map(func, lines))
        t += time()
        print(f'total {t:.6f}:', total)
    print()
