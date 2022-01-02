class Node:
    def __init__(self , label, is_variable = False, is_function = False):
        self.childs = []
        self.label = label
        self.operator = False
        self.is_variable = is_variable
        self.is_function = is_function
        if label == "+":
            self.operator = True
        elif label == "-":
            self.operator = True
        elif label == "*":
            self.operator = True
        elif label == "/":
            self.operator = True