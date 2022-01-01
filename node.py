class Node:
    def __init__(self , label, is_variable = False):
        self.left = None
        self.right = None
        self.label = label
        self.operator = False
        self.is_variable = is_variable
        if label == "+":
            self.operator = True
        elif label == "-":
            self.operator = True
        elif label == "*":
            self.operator = True
        elif label == "/":
            self.operator = True