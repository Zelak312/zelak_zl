"""
a + 1
1 + 1
a = 1 + 2
pl(a)
for x in 5 {

}

1 + 2 + 3 + 4
-|

Scope = (Loop|Condition|Statement)*
Loop = "for"+VariableName+Number+"{"+(Scope)*+"}"
Condition = "if"+Conditional+"{"+(Scope)*+"}"
Conditional = Math+(<|>|==|!=)+Math
Statement = (Function|Expression|Math)
Function = AnyType"("+AnyType+")"
Expression(AnyType) = AnyType+"="+Math
Math(AnyType) = PowDiv+("+"|"-"+PowDiv)*
PowDiv(AnyType) = Parenthese+("*"|"/"+Parenthese)*
Parenthese(AnyType) = ("("+Math+")")|AnyType
---
AnyType = VariableName|String|Number
VariableName = (Alphabet+(Number+Alphabet)*)
"""

from parser import Parser
from node import Node
from tree import evaluate_node

class CustomParser(Parser):
    def start(self, text):
        self.pos = 0
        self.text = text
        return self.scope()

    # Scope = (Loop|Condition|Statement)*
    def scope(self):
        root = Node("scope", scope=True)
        while True:
            try:
                root.childs.append(self.loop())
            except:
                try:
                    root.childs.append(self.condition())
                except:
                    try:
                        root.childs.append(self.statement())
                    except:
                        break

        return root

    # Loop = "for"+VariableName+Number+"{"+(Scope)*+"}"
    def loop(self):
        self.keyword(["for"])
        var_name = self.variable_name()
        self.keyword(["in"])
        var_loop_time = self.number()
        self.keyword(["{"])
        scope = self.scope()
        self.keyword(["}"])

        root = Node("for")
        root.childs.append(var_name)
        root.childs.append(var_loop_time)
        root.childs.append(scope)
        return root

    # Condition = "if"+Conditional+"{"+(Scope)*+"}"
    def condition(self):
        self.keyword(["if"])
        conditional = self.conditional()
        self.keyword(["{"])
        scope = self.scope()
        self.keyword(["}"])

        root = Node("if")
        root.childs.append(conditional)
        root.childs.append(scope)
        return root

    # Conditional = Math+(<|>|==|!=)+Math
    def conditional(self):
        left = self.math()
        op = Node(self.keyword(["<", ">", "==", "!="]))
        right = self.math()

        op.childs.append(left)
        op.childs.append(right)
        return op

    # Statement = (Function|Expression|Math)
    def statement(self):
        result = self.function()
        if result[0]:
            return result[1]

        result = self.expression(result[1])
        if result[0]:
            return result[1]

        return self.math(result[1])

    # Function = AnyType"("+AnyType+")"
    def function(self):
        func_name = self.any_type()
        if self.maybe_keyword(["("]) is None:
            return (False, func_name)
        argument = self.any_type()
        self.keyword([")"])
        func_name.is_function = True
        func_name.childs.append(argument)
        return (True, func_name)

    # Expression = AnyType+"="+Math
    def expression(self, any_type = None):
        if any_type == None:
            var_name = self.any_type()
        else:
            var_name = any_type
        if self.maybe_keyword(["="]) is None:
            return (False, var_name)
        content = self.math()
        root = Node("=")
        # Make the variable type known
        var_name.is_string = content.is_string
        root.childs.append(var_name)
        root.childs.append(content)
        return (True, root)

    # Math = PowDiv+("+"|"-"+PowDiv)*
    def math(self, any_type = None):
        result = self.pow_div(any_type)
        op = self.maybe_keyword(["+", "-"])
        if op != None:
            root = Node(op)
            root.childs.append(result)
            root.childs.append(self.math())
            return root
        
        return result

    # PowDiv = Parenthese+("*"|"/"+Parenthese)*
    def pow_div(self, any_type = None):
        result = self.parenthese(any_type)
        op = self.maybe_keyword(["*", "/"])
        if op != None:
            root = Node(op)
            root.childs.append(result)
            root.childs.append(self.pow_div())
            return root
        
        return result

    # Parenthese = ("("+Math+")")|AnyType
    def parenthese(self, any_type = None):
        if self.maybe_keyword(["("]):
            math = self.math()
            self.keyword([")"])
            return math
        elif any_type != None:
            return any_type
        return self.any_type()

    # AnyType = VariableName|String|Number
    def any_type(self):
        try:
            return self.variable_name()
        except:
            try:
                return self.string()
            except:
               return self.number()

    # VariableName = (Alphabet+(Number|Alphabet)*)
    def variable_name(self):
        var_name = ""
        found = self.char(["A-Z", "a-z", "_"])
        while found != None:
            var_name += found
            found = self.maybe_char(["A-Z", "a-z", "_", "0-9"])
        
        return Node(var_name, variable=True)

    # String
    def string(self):
        all_chars = [char for char in "!#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ "]
        all_chars.append("A-Z")
        all_chars.append("a-z")
        all_chars.append("0-9")
        string = ""
        self.char(["\""]) # Yes I only support " for now
        found = self.char(all_chars)
        while found != None:
            string += found
            found = self.maybe_char(all_chars)
            if found == "\\":
                # Escape sequance
                to_escape = self.char()
                if to_escape == "t":
                    found = "\t"
                else:
                    found = to_escape

        self.keyword(["\""])
        return Node(string, string=True)

    # Number
    def number(self):
        number = ""
        found = self.char(["0-9", "."])
        is_float = False
        while found != None:
            number += found
            found = self.maybe_char(["0-9", "."])
            if found == ".":
                is_float = True
        if is_float:
            number = float(number)
        else:
            number = int(number)

        return Node(number)

parser = CustomParser()
var_table = {}

with open('test.zl') as f:
    code = f.read()
    ast = parser.start(code)
    evaluate_node(ast, var_table)