"""
Expression = Assignation|Math
Assignation = alphabetics+"="+Math

Math = PowDiv+("+"|"-"+PowDiv)*
PowDiv = Parenthese+("*"|"/"+Parenthese)*
Parenthese = ("("+Math+")")|Input

Input = Numbers|Alphabetics
Numbers = numbers
Alphabetics = alphabetics
"""

from parser import Parser
from node import Node
from tree import evaluate_node

class CustomParser(Parser):
    def start(self, text):
        self.pos = 0
        self.text = text
        return self.expression()

    def expression(self):
        try:
            found = self.assignation()
        except:
            found = self.math()

        return found

    def assignation(self):
        left = Node(self.alphabetics())
        root = Node(self.keyword(["="]))
        root.left = left
        root.right = self.math()

        return root

    def math(self):
        left = self.pow_div()
        op = self.maybe_keyword(["+", "-"])
        if op != None:
            root = Node(op)
            root.left = left
            root.right = self.math()
            return root
        
        return left

    def pow_div(self):
        left = self.parenthese()
        op = self.maybe_keyword(["*", "/"])
        if op != None:
            root = Node(op)
            root.left = left
            root.right = self.pow_div()
            return root
        
        return left

    def parenthese(self):
        try:
            self.keyword(["("])
            math = self.math()
            self.keyword([")"])
            return math
        except:
            return self.input()

    def input(self):
        try:
            return Node(self.numbers())
        except:
            return Node(self.alphabetics(), True)
    
    def numbers(self):
        number = ""
        found = self.char(["0-9"])
        while found != None:
            number += found
            found = self.maybe_char(["0-9"])

        return float(number)

    def alphabetics(self):
        alpha = ""
        found = self.char(["A-Z", "a-z"])
        while found != None:
            alpha += found
            found = self.maybe_char(["A-Z", "a-z"])

        return alpha

parser = CustomParser()
var_table = {}

while True:
    t = input()
    # Parse input and create ast
    ast = parser.start(t)
    # evaluate ast
    result = evaluate_node(ast, var_table)
    print("> " + str(result))
