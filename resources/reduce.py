
import argparse 
import math
import operator as op


def tokenize(code):
    tokens = []
    token = ""
    i = 0
    while i < len(code):
        c = code[i]
        # ( # )
        if c in ("(", ")"):
            if token:
                tokens.append(token)
                token = ""
            tokens.append(c)
        elif c.isspace():
            if token:
                tokens.append(token)
                token = ""
        # handle comments
        elif c == ";":
            #  iterate i until c is newline
            while c != "\n" and i < len(code):
                i += 1
                c = code[i]
        else:
            token += c
        i += 1
    if token:
        tokens.append(token)
    return tokens

def parse(tokens, i=0):
    expr = []
    while i < len(tokens):
        token = tokens[i]
        if token == "(":
            e, i = parse(tokens, i+1)
            expr.append(e)
        elif token == ")":
            return expr, i+1
        else:
            i += 1
            if token.isnumeric():
                token = float(token)
            expr.append(token)
    return expr[0]

def fn(name, body, env):
    def innerfn():
        eval(body, env)
    env[name] = innerfn

class Env(dict):
    def __init__(self, keys=(), vals=(), parent=None):
        self.update(zip(keys, vals))
        self.parent = parent

    def find(self, key):
        """returns the innermost dictionary in which the key occurs"""
        target = self
        while target:
            if key in target:
                return target
            else:
                target = self.parent
        return None

class Function():
    def __init__(self, params, body, env):
        self.params, self.body, self.env = params, body, env
    def __call__(self, *args): 
        return eval(self.body, Env(self.params, args, self.env))

def default_env():
    env = Env()
    return env

def eval(expr, env=default_env()):
    # its a number
    if isinstance(expr, float):
        return expr
    
    # its a token
    if type(expr) == str:
        symbol = expr
        return env.find(symbol)[symbol]
        
    # its not an atomic expression yet, evaluate all pieces
    if type(expr) == list:
        fs, args = expr[0], expr[1:]

        # special forms
        if fs == "def":
            symbol, exp = args
            env[symbol] = eval(exp, env)
            return 
        elif fs == "cond":
            for condition, exp in args:
                if condition == "True" or eval(condition, env):
                    return eval(exp, env)
        elif fs == "quote":
            return args[0]
        elif fs == "set":
            symbol, exp = args
            env.find(symbol)[symbol] = eval(exp, env)
            return 
        elif fs == "fn":
            params, body = args
            return Function(params, body, env)
        else:
            # procedure call
            fn = eval(fs, env)
            args = [eval(arg, env) for arg in args]
            res = fn(*args)
            return res


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description='a lisp interpreter')
    parser.add_argument(
        'input_file', type=str,
        help='the file to load the code from')
    args = parser.parse_args()
    print(args.input_file)

    program = None
    with open(args.input_file, "r") as file:
        program = file.read()

    if program is None:
        raise Exception("uh what. no program bruh")

    tokens = tokenize(program)
    print(tokens)
    ast = parse(tokens)
    print(ast)
    tl_env = default_env()
    res = eval(ast, tl_env)
    print(res)