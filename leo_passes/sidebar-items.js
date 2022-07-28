initSidebarItems({"mod":[["code_generation",""],["loop_unrolling",""],["pass",""],["static_single_assignment","The Static Single Assignment pass traverses the AST and converts it into SSA form. See https://en.wikipedia.org/wiki/Static_single-assignment_form for more information. The pass also flattens `ConditionalStatement`s into a sequence of `AssignStatement`s. The pass also rewrites complex `AssignStatement`s into simpler ones. e.g x += 1 -> x = x + 1. The pass also rewrites `ReturnStatement`s into `AssignStatement`s and consolidates the returned values as a single `ReturnStatement` at the end of the function."],["symbol_table",""],["type_checking",""]]});