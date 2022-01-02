class Node:
    def __init__(self , label, **kwargs):
        self.childs = []
        self.label = label
        self.operator = False
        self.is_scope = kwargs.get("scope", False)
        self.is_variable = kwargs.get("variable", False)
        self.is_string = kwargs.get("string", False)
        self.is_function = kwargs.get("function", False)
        if label == "+":
            self.operator = True
        elif label == "-":
            self.operator = True
        elif label == "*":
            self.operator = True
        elif label == "/":
            self.operator = True