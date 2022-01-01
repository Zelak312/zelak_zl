def evaluate_node(node, var_table):
    if node.label == "=":
        var = node.left.label
        var_table[var] = evaluate_node(node.right, var_table)
        return var_table[var]
    elif node.operator:
        if node.label == "+":
            return evaluate_node(node.left, var_table) + evaluate_node(node.right, var_table)
        elif node.label == "-":
            return evaluate_node(node.left, var_table) - evaluate_node(node.right, var_table)
        elif node.label == "*":
            return evaluate_node(node.left, var_table) * evaluate_node(node.right, var_table)
        elif node.label == "/":
            return evaluate_node(node.left, var_table) / evaluate_node(node.right, var_table)
    else:
        if node.is_variable:
            return var_table[node.label]
        else:
            return int(node.label)