def evaluate_node(node, var_table):
    if node.is_scope:
        scopes = node.childs
        for scope in scopes:
            evaluate_node(scope, var_table)
    elif node.label == "=":
        var = node.childs[0].label
        var_table[var] = evaluate_node(node.childs[1], var_table)
        return var_table[var]
    elif node.label == "for":
        counter_name = node.childs[0].label
        times_to_run = node.childs[1].label
        for x in range(int(times_to_run)):
            var_table[counter_name] = x
            evaluate_node(node.childs[2], var_table)
    elif node.is_function:
        func_name = node.label
        if func_name == "print_var":
            print(evaluate_node(node.childs[0], var_table))
    elif node.operator:
        if node.label == "+":
            left = evaluate_node(node.childs[0], var_table)
            right = evaluate_node(node.childs[1], var_table)
            if type(left) is str or type(right) is str:
                return str(left) + str(right)
            return left + right
        elif node.label == "-":
            left = evaluate_node(node.childs[0], var_table)
            right = evaluate_node(node.childs[1], var_table)
            if type(left) is str or type(right) is str:
                raise Exception("Cannot use '-' operator with string")
            return left - right
        elif node.label == "*":
            left = evaluate_node(node.childs[0], var_table)
            right = evaluate_node(node.childs[1], var_table)
            if type(left) is str or type(right) is str:
                raise Exception("Cannot use '*' operator with string")
            return left * right
        elif node.label == "/":
            left = evaluate_node(node.childs[0], var_table)
            right = evaluate_node(node.childs[1], var_table)
            if type(left) is str or type(right) is str:
                raise Exception("Cannot use '/' operator with string")
            return left / right
    else:
        if node.is_variable:
            return var_table[node.label]
        return node.label