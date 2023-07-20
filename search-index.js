var searchIndex = JSON.parse('{\
"wabbit":{"doc":"This is my implementation of Wabbit, written as part of …","t":"CNNNNCNNNNCCCENCEAAAAAAAAAAAAAFFADLLMLLLMMLLLLLMLLMLMMLMLLLLLLNNNNNENNNNNNNNNENNNNNNLLLLLLLLLLLLLLLLLLLLLLLLLLLLMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMDNNELLLLLLLLLLLLLLLLLLLLLLLLLMLLLLLLLLLMNNNNNNNNNNNNNNNNNNNNENNNINNNGGNNNNNNNNNDDLLLLLLLLLLOMLKLLLLLLLLLMLOLLMLMMMLLLLLLLLLLLLNNGNENLLLLLLLLLLLLNDNRNENMLLLLLMMMLLLLLLMMMLLLLLLLMLLLLLLLLMENNNNNNNNNNENENNNNNNELLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLDLLLLLLLLLLLLLLMLLLLLLLLLLLLMLLLLLLLLLLLLMLLMLLMLLLLLLDHHHHDDDDMMMMFFFFFFFFLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLMLLLLLLMMMLLLLLLLLLLLLLLLNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNDENNNNLLLLLLLLLLLLLLLLLMMMMLLMLLLLLLGLNNNNNNNNENELLLLLLLLLLOLLLOLLLLLLLLLLLLLLLLLLLLLOOLLLLLLLLLLRFFFF","n":["Analyzer","Bool","Bool","Char","Char","CodegenLLVM","Float","Float","Int","Int","Interpreter","Parser","Scanner","Type","TypeHolder","Typechecker","WabbitType","analyzer","ast","environment","error","formatter","interpreter","llvm","operators","parser","scanner","tokens","typechecker","types","wasm_entry","wasm_interp","wasm_interpreter","Analyzer","borrow","borrow_mut","call_depth","check_constant","check_env","check_function","constants","env","evaluate","expr_type","fmt","from","from","functions","interpret","into","loop_depth","new","output","ranges","run_stmt","statements","try_from","try_into","type_id","typecheck","typecheck_bool","typecheck_stmt","Assign","Binary","Block","Call","ConstDef","Expr","Expr","FuncDef","Grouping","If","Literal","Logical","LoopControl","Print","Return","Stmt","TypeConversion","TypeName","Unary","VarDef","VarName","While","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","eq","eq","fmt","fmt","fmt","fmt","from","from","into","into","to_owned","to_owned","to_string","to_string","try_from","try_from","try_into","try_into","type_id","type_id","dtype","dtype","e","id","id","id","id","id","id","id","id","id","lhs","lhs","name","name","op","op","op","operand","params","params","rhs","rhs","value","body","body","condition","condition","control","def_name","def_params","id","id","id","id","id","id","id","id","id","id","maybe_else_block","maybe_type","maybe_type","maybe_value","name","name","name","return_type","statements","then_block","value","value","value","value","Environment","Init","UnInit","VarStore","assign","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","clone_store","define_init","define_uninit","enter_child","enter_child_fn","exit_child","exit_child_unwrap","fmt","fmt","from","from","get","in_global_scope","into","into","new","parent","to_owned","to_owned","top_contains","try_from","try_from","try_into","try_into","type_id","type_id","values","AccessUninit","AltBranch","AssignRetype","AssignUndefined","ConstScope","ConvertAirty","DoubleToken","DupArgs","ExpectExpr","ExpectType","ExpectTypeName","ExpectVarName","FuncAirty","FuncDefScope","FuncUndefined","InitType","InternalErr","InvalidChar","InvalidNumber","LoopReq","Msg","NoReturn","ParamType","ParserExpect","RangeReporter","RedeclareConst","RedeclareFunc","RedeclareVar","Result","Results","ReturnDiverge","ReturnScope","ReturnType","TypeConvert","TypeEval","TypeMatch","UnexpectedChar","VarDefEmpty","VarUndefined","WabbitError","WabbitErrorReporter","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","err","errors","extract_range","extract_tokens","fmt","fmt","fmt","from","from","from","into","into","into","label","msg","msg","new","new","path","provide","range","source","title","to_owned","to_owned","to_string","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","Break","Continue","Interpreter","Return","Signal","Unit","borrow","borrow_mut","evaluate","fmt","from","interpret","into","run_stmt","try_from","try_into","type_id","typecheck_bool","Break","CodegenLLVM","Continue","RUNTIME","Return","Signal","Unit","analyze","binary_ops","borrow","borrow","borrow_mut","borrow_mut","break_labels","continue_labels","counter","enter_child","eq","exit_child_unwrap","from","from","from","func_llvm","global_vars","globals","into","into","label_name","llvm_codegen","llvm_expr","llvm_stmt","loc","main","tmp_name","tmp_no_inc","try_from","try_from","try_into","try_into","type_id","type_id","var_names","BinaryOp","Break","Continue","Divide","EqualEqual","Greater","GreaterEqual","Less","LessEqual","LogicalAnd","LogicalNot","LogicalOp","LogicalOr","LoopControl","Minus","Minus","NotEqual","Plus","Plus","Times","UnaryOp","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","eq","eq","eq","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","into","into","into","into","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","Parser","add_or_sub","advance","and","assign_id","assign_id_single","block","borrow","borrow_mut","borrow_ranges","borrow_statements","call","check","compare","constdef","current","expect","expr_stmt","expression","extract_range","extract_tokens","finish_call","fmt","from","from","funcdef","get_name","get_type","id","if_stmt","into","is_end","loop_control","match_any","new","or","parse","peek","previous","primary","print_stmt","ranges","return_stmt","statement","statements","stmt_name","times_or_div","tokens","try_from","try_into","type_id","unary","vardef","while_stmt","KEYWORDS","LAZY","LAZY","LAZY","LAZY","Scanner","TOKENS_DOUBLE","TOKENS_SINGLE","TYPES","__private_field","__private_field","__private_field","__private_field","__stability","__stability","__stability","__stability","__static_ref_initialize","__static_ref_initialize","__static_ref_initialize","__static_ref_initialize","add_literal_token","add_token","advance","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_tokens","current","deref","deref","deref","deref","extract_range","extract_tokens","fmt","from","from","from","from","from","identifier","into","into","into","into","into","is_end","lexeme","line","new","number","peek","peek_next","scan","scan_token","source","start","tokens","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","Assign","BoolType","Break","Char","CharType","Comma","Const","Continue","Divide","Else","Eof","EqualEqual","False","Float","FloatType","Func","Greater","GreaterEqual","If","Integer","IntegerType","LeftBrace","LeftParen","Less","LessEqual","LogicalAnd","LogicalNot","LogicalOr","Minus","Name","NotEqual","Plus","Print","Return","RightBrace","RightParen","Semicolon","Times","Token","TokenType","True","Type","Var","While","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","eq","extract_range","extract_tokens","fmt","fmt","from","from","into","into","lexeme","line","literal","range","to_owned","to_owned","token","try_from","try_from","try_into","try_into","type_id","type_id","Typechecker","typecheck","Bool","Bool","Char","Char","Float","Float","Int","Int","Type","TypeHolder","WabbitType","bool_compare","borrow","borrow","borrow_mut","borrow_mut","char_compare","clone","clone","clone_into","clone_into","compare","dtype","eq","eq","equality","float_binary","float_compare","float_unary","fmt","fmt","fmt","fmt","from","from","from","from","from","from","global_init","int_binary","int_compare","int_unary","into","into","llvm_type","llvm_value","numeric_binary","numeric_unary","to_owned","to_owned","to_string","to_string","try_from","try_from","try_into","try_into","type_id","type_id","_ASSERT","__wasm_bindgen_generated_wasm_entry","__wasm_bindgen_generated_wasm_interp","wasm_entry","wasm_interp"],"q":["wabbit","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","wabbit::analyzer","","","","","","","","","","","","","","","","","","","","","","","","","","","","","wabbit::ast","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","wabbit::ast::Expr","","","","","","","","","","","","","","","","","","","","","","","","","wabbit::ast::Stmt","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","wabbit::environment","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","wabbit::error","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","wabbit::interpreter","","","","","","","","","","","","","","","","","","wabbit::llvm","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","wabbit::operators","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","wabbit::parser","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","wabbit::scanner","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","wabbit::tokens","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","wabbit::typechecker","","wabbit::types","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","wabbit::wasm_interpreter","","","",""],"d":["","","","","","","","","","","","","","Wabbit data types","","","a Wabbit value, including Wabbit types","typecheck or interpret Wabbit AST","types for Wabbit AST","manage variable scope","error reporting","a code minimizer","interpret Wabbit AST","generate LLVM IR","Wabbit primitive operators","parse Wabbit tokens","scan Wabbit source code","Wabbit tokens","interpret Wabbit AST","Wabbit types and values","entry point for WebAssembly interpreter","a Wabbit interpreter, exported to WebAssembly","a <code>WebAssembly</code> interpreter","struct for typechecking or interpreting Wabbit AST","","","current depth of function calls","check if a name is already used by a constant","check if a name is already used by a variable name in the …","check if a name is already used by a function","map of constants to types or values","map of variables to types or values","interpret a single expression","typecheck a single expression","","","Returns the argument unchanged.","map of functions to types or values","interpret all statements","Calls <code>U::from(self)</code>.","current depth of while loops","","record of IO (print statements)","map of expressions/statemts to source indices, borrowed …","interpret a single statement","statements, borrowed from a parser","","","","typecheck all statements","interpret an expression and confirm it is a boolean at …","typecheck a single statement","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","collection of various errors that could be raised by an …","","","","trait for reporting the left and right source indices of …","","","","","","","","","","","","","","","an error, pointing to source code indices","struct for reporting multiple errors","","","","","","","","","","","convenience macro for construction of a <code>WabbitError</code>","vector of errors","use <code>extract_tokens</code> to get tokens, then find the left and …","extract tokens from a general type","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","error text","an error message template","convenience macro for raising errors","","","path to source file","","source code indices","raw source code string","stage at which error occurred","","","","","","","","","","","","","","","struct for interpreting Wabbit AST","","","","","","interpret a single expression","","Returns the argument unchanged.","interpret all statements","Calls <code>U::from(self)</code>.","interpret a single statement","","","","interpret an expression and confirm it is a boolean at …","","","","","","","","a typechecker","utility for generating binary instructions","","","","","stack of labels that a break statement could jump to","stack of labels that a continue statement could jump to","used to generate unique names","enter child environment for variable names and types","","exit child environment for variable names and types","","Returns the argument unchanged.","Returns the argument unchanged.","raw LLVM strings for functions","set of LLVM globals, for any variable defined in global …","raw LLVM strings in the global space","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","generate a unique label name","transform AST into LLVM IR","generate LLVM for a single expression","generate LLVM for a single statement","a reference to either the LLVM main or global space","raw LLVM strings in the main function","generate a unique variable name","get the current variable name","","","","","","","map of Wabbit variable names to LLVM variable names","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","Struct for transforming tokens into a vector of statements …","","return the current token and advance the parser one token","","given a pair of tokens/expressions/statements, record the …","record the source range of a single token","","","","return a reference to <code>self.ranges</code>","return a reference to <code>self.statements</code>","","check if the current token matches","","","index of <code>self.tokens</code> that the parser is examining","advance past and return a specific token, or return an …","","","","","","","Returns the argument unchanged.","","","advance past and return a name, or return an error","advance past and return a type, or return an error","current expression/statement index","","Calls <code>U::from(self)</code>.","check if all statements have been parsed","","check if any token is a match, and if so advance","","","parse all statements","return the current token without advancing the parser","return the previous token (or current if at the first …","","","a map from statement/expression indices to source indices","","parse a single statement","resulting statements after parsing","","","tokens, borrowed from a scanner","","","","","","","","","","","","Struct for transforming the raw character input of a …","","","","","","","","","","","","","","","","add a literal token (string, number, or identifier) to …","add a non-literal token to <code>self.tokens</code>","return the current character and advance the scanner one …","","","","","","","","","","","get a reference to a scanner’s tokens","index of <code>self.source</code> that the scanner is examining","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","scan an identifier","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","check if all characters have been scanned","get the current range of token characters as a string","current line number the scanner is examining","initialize a new scanner","scan a numeric literal (integer or float)","return the current character without advancing the scanner","return the following character without advancing the …","scan all tokens","scan a single token","raw character input of a Wabbit program","starting index before scanner gets next token","resulting tokens after scanning <code>self.source</code>","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","struct for typechecking Wabbit AST","typecheck all statements","","","","","","","","","Wabbit data types","","a Wabbit value, including Wabbit types","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","","","","Returns the argument unchanged.","","transform a type into an LLVM string for zero","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","transform a Wabbit type into an LLVM string","transform a Wabbit value into an LLVM string","","","","","","","","","","","","","","entry point for WebAssembly interpreter","a Wabbit interpreter, exported to WebAssembly","entry point for WebAssembly interpreter","a Wabbit interpreter, exported to WebAssembly"],"i":[0,10,8,10,8,0,10,8,10,8,0,0,0,0,8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,16,9,16,9,16,0,16,16,9,16,9,9,16,16,16,0,9,9,9,16,9,16,16,9,16,9,16,9,16,9,16,9,16,16,9,9,16,9,16,9,16,9,16,9,16,9,16,9,16,9,50,51,52,53,50,54,55,51,52,56,57,58,54,56,53,55,54,56,57,57,53,50,54,56,58,59,60,61,60,62,59,59,63,64,65,59,61,60,62,66,67,68,61,64,65,64,64,65,67,59,68,61,63,65,66,67,0,23,23,0,21,23,21,23,21,23,21,23,21,23,21,21,21,21,21,21,23,21,23,21,21,21,23,21,21,21,23,21,21,23,21,23,21,23,21,21,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,0,27,27,27,0,27,27,27,0,0,27,27,27,27,27,27,27,27,27,0,0,27,25,26,27,25,26,25,26,25,26,0,26,69,69,25,26,26,27,25,26,27,25,26,25,27,0,25,26,26,26,25,26,26,25,26,26,27,25,26,27,25,26,27,25,26,17,17,0,17,0,17,17,17,30,17,17,30,17,30,17,17,17,30,33,0,33,0,33,0,33,31,31,31,33,31,33,31,31,31,31,33,31,31,31,33,31,31,31,31,33,31,31,31,31,31,31,31,31,31,33,31,33,31,33,31,0,36,36,32,32,32,32,32,32,35,34,0,35,0,32,34,32,32,34,32,0,32,34,35,36,32,34,35,36,32,34,35,36,32,34,35,36,32,34,35,36,32,32,34,34,35,35,36,32,34,35,36,32,34,35,36,32,34,35,36,32,34,35,32,32,34,34,35,35,36,36,32,34,35,36,32,34,35,36,0,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,0,0,0,0,0,0,0,0,0,41,42,43,44,0,0,0,0,0,0,0,0,40,40,40,41,42,43,44,40,41,42,43,44,40,40,40,41,42,43,44,40,40,40,41,42,43,44,40,40,41,42,43,44,40,40,40,40,40,40,40,40,40,40,40,40,40,41,42,43,44,40,41,42,43,44,40,41,42,43,44,40,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,38,0,0,38,38,38,38,38,37,38,37,38,37,38,37,38,37,37,38,37,38,37,38,37,37,37,37,37,38,37,37,38,37,38,37,38,37,0,46,10,8,10,8,10,8,10,8,0,8,0,8,10,8,10,8,8,10,8,10,8,0,8,10,8,0,8,8,8,10,10,8,8,10,8,8,8,8,8,10,8,8,8,10,8,10,8,0,0,10,8,10,8,10,8,10,8,10,8,0,0,0,0,0],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[],[[2,[1]]]],[3,4],0,0,[[]],[[]],0,[[5,4,6],7],[[5,4,6],7],[[5,4,6],7],0,0,[[[5,[8]],9],[[7,[8]]]],[[[5,[10]],9],[[7,[10]]]],[[5,11],12],[13,5],[[]],0,[[[5,[8]]],7],[[]],0,[[14,15],5],0,0,[[[5,[8]],16],[[7,[17]]]],0,[[],2],[[],2],[[],18],[[[5,[10]]],7],[[[5,[8]],9,6],[[7,[19]]]],[[[5,[10]],16],[[7,[[20,[10]]]]]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[16,16],[9,9],[[]],[[]],[[16,16],19],[[9,9],19],[[16,11],12],[[16,11],12],[[9,11],12],[[9,11],12],[[]],[[]],[[]],[[]],[[]],[[]],[[],4],[[],4],[[],2],[[],2],[[],2],[[],2],[[],18],[[],18],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[21,4]],[[]],[[]],[[]],[[]],[[[23,[22]]],[[23,[22]]]],[21,21],[[]],[[]],[23],[[21,4]],[[21,4]],[21],[[21,[15,[4,23]]]],[21,7],[21],[[[23,[24]],11],12],[[21,11],12],[[]],[[]],[[21,4],[[20,[23]]]],[21,19],[[]],[[]],[[],21],0,[[]],[[]],[[21,4],19],[[],2],[[],2],[[],2],[[],2],[[],18],[[],18],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[25,25],[26,26],[[]],[[]],0,0,[[]],[14],[[25,11],12],[[26,11],12],[[26,11],12],[[]],[[]],[[]],[[]],[[]],[[]],0,[27,3],0,[[],25],[[[14,[25]],28,4,3],26],0,[29],0,0,0,[[]],[[]],[[],4],[[],2],[[],2],[[],2],[[],2],[[],2],[[],2],[[],18],[[],18],[[],18],0,0,0,0,0,0,[[]],[[]],[[30,9],[[7,[8]]]],[[17,11],12],[[]],[30,7],[[]],[[30,16],[[7,[17]]]],[[],2],[[],2],[[],18],[[30,9,6],[[7,[19]]]],0,0,0,0,0,0,0,0,[[31,10,32],4],[[]],[[]],[[]],[[]],0,0,0,[31],[[33,33],19],[31],[13,31],[[]],[[]],0,0,0,[[]],[[]],[[31,3],4],[31,4],[[31,9],4],[[31,16],33],[31,14],0,[31,4],[31,4],[[],2],[[],2],[[],2],[[],2],[[],18],[[],18],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[32,32],[34,34],[35,35],[36,36],[[]],[[]],[[]],[[]],[[32,32],19],[[34,34],19],[[35,35],19],[[36,36],19],[[32,11],12],[[32,11],12],[[34,11],12],[[34,11],12],[[35,11],12],[[35,11],12],[[36,11],12],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],4],[[],4],[[],4],[[],2],[37,[[2,[32]]]],[37,[[2,[34]]]],[[],2],[37,[[2,[35]]]],[[],2],[[],2],[37,[[2,[36]]]],[[],2],[[],2],[[],2],[[],2],[[],18],[[],18],[[],18],[[],18],0,[13,[[7,[9]]]],[13,37],[13,[[7,[9]]]],[13,6],[[13,37],6],[13,[[7,[16]]]],[[]],[[]],[13,15],[13,14],[13,[[7,[9]]]],[[13,38],19],[13,[[7,[9]]]],[13,[[7,[16]]]],0,[[13,39],[[7,[37]]]],[13,[[7,[16]]]],[13,[[7,[9]]]],[13],[[13,14]],[[13,9,37],[[7,[9]]]],[[13,11],12],[[]],[40,13],[13,[[7,[16]]]],[13,[[7,[37]]]],[13,7],0,[13,[[7,[16]]]],[[]],[13,19],[13,[[7,[16]]]],[13,19],[14,13],[13,[[7,[9]]]],[13,7],[13,37],[13,37],[13,[[7,[9]]]],[13,[[7,[16]]]],0,[13,[[7,[16]]]],[13,[[7,[16]]]],0,[13,[[7,[16]]]],[13,[[7,[9]]]],0,[[],2],[[],2],[[],18],[13,[[7,[9]]]],[13,[[7,[16]]]],[13,[[7,[16]]]],0,0,0,0,0,0,0,0,0,0,0,0,0,[[],15],[[],15],[[],15],[[],15],[[],[[15,[39]]]],[[],[[15,[39,38]]]],[[],[[15,[3,38]]]],[[],[[15,[3,38]]]],[[40,38,8]],[[40,38]],[40,39],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[40,14],0,[41,15],[42,15],[43,15],[44,15],[40],[[40,14]],[[40,11],12],[[]],[[]],[[]],[[]],[[]],[40],[[]],[[]],[[]],[[]],[[]],[40,19],[40,4],0,[3,40],[[40,19],7],[40,39],[40,39],[40,45],[40,45],0,0,0,[[],2],[[],2],[[],2],[[],2],[[],2],[[],2],[[],2],[[],2],[[],2],[[],2],[[],18],[[],18],[[],18],[[],18],[[],18],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[38,38],[37,37],[[]],[[]],[[38,38],19],[37],[[37,14]],[[38,11],12],[[37,11],12],[[]],[[]],[[]],[[]],0,0,0,0,[[]],[[]],0,[[],2],[[],2],[[],2],[[],2],[[],18],[[],18],0,[46,7],0,0,0,0,0,0,0,0,0,0,0,[[8,8,47],8],[[]],[[]],[[]],[[]],[[8,8,47],8],[10,10],[8,8],[[]],[[]],0,[8,10],[[10,10],19],[[8,8],19],0,[[8,8,47],8],[[8,8,47],8],[[8,47],8],[[10,11],12],[[10,11],12],[[8,11],12],[[8,11],12],[[]],[48,8],[49,8],[39,8],[[]],[19,8],[10,4],[[8,8,47],8],[[8,8,47],8],[[8,47],8],[[]],[[]],[10,4],[8,4],0,0,[[]],[[]],[[],4],[[],4],[[],2],[[],2],[[],2],[[],2],[[],18],[[],18],0,[[]],[[]],[[],[[2,[1]]]],[3,4]],"p":[[3,"JsValue"],[4,"Result"],[15,"str"],[3,"String"],[3,"Analyzer"],[15,"usize"],[6,"Result"],[4,"WabbitType"],[4,"Expr"],[4,"Type"],[3,"Formatter"],[6,"Result"],[3,"Parser"],[3,"Vec"],[3,"HashMap"],[4,"Stmt"],[4,"Signal"],[3,"TypeId"],[15,"bool"],[4,"Option"],[3,"Environment"],[8,"Clone"],[4,"VarStore"],[8,"Debug"],[3,"WabbitError"],[3,"WabbitErrorReporter"],[4,"Msg"],[3,"PathBuf"],[3,"Demand"],[6,"Interpreter"],[3,"CodegenLLVM"],[4,"BinaryOp"],[4,"Signal"],[4,"UnaryOp"],[4,"LogicalOp"],[4,"LoopControl"],[3,"Token"],[4,"TokenType"],[15,"char"],[3,"Scanner"],[3,"TOKENS_SINGLE"],[3,"TOKENS_DOUBLE"],[3,"KEYWORDS"],[3,"TYPES"],[6,"Results"],[6,"Typechecker"],[8,"Fn"],[15,"f64"],[15,"i32"],[13,"TypeConversion"],[13,"TypeName"],[13,"Grouping"],[13,"Call"],[13,"Logical"],[13,"VarName"],[13,"Binary"],[13,"Unary"],[13,"Literal"],[13,"FuncDef"],[13,"While"],[13,"If"],[13,"LoopControl"],[13,"Print"],[13,"VarDef"],[13,"ConstDef"],[13,"Return"],[13,"Assign"],[13,"Block"],[8,"RangeReporter"]]},\
"wabbit_rs":{"doc":"","t":"DMLLLLLLLLLLLMLMMFMMMMLLLLL","n":["Cli","ast","augment_args","augment_args_for_update","borrow","borrow_mut","command","command_for_update","fmt","from","from_arg_matches","from_arg_matches_mut","group_id","interpret","into","llvm_exec","llvm_print","main","minimize","path","skip_typecheck","tokens","try_from","try_into","type_id","update_from_arg_matches","update_from_arg_matches_mut"],"q":["wabbit_rs","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["command-line options","option to print AST","","","","","","","","Returns the argument unchanged.","","","","option to use interpreter","Calls <code>U::from(self)</code>.","option to compile and execute LLVM IR (using clang)","option to print LLVM IR","","option to print minimized code","path to Wabbit program","option to skip typechecking","option to print tokens","","","","",""],"i":[0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,2,2,2,2,2,2,2,2,2],"f":[0,0,[1,1],[1,1],[[]],[[]],[[],1],[[],1],[[2,3],4],[[]],[5,[[7,[2,6]]]],[5,[[7,[2,6]]]],[[],[[9,[8]]]],0,[[]],0,0,[[],[[7,[10]]]],0,0,0,0,[[],7],[[],7],[[],11],[[2,5],[[7,[6]]]],[[2,5],[[7,[6]]]]],"p":[[3,"Command"],[3,"Cli"],[3,"Formatter"],[6,"Result"],[3,"ArgMatches"],[6,"Error"],[4,"Result"],[3,"Id"],[4,"Option"],[3,"MainError"],[3,"TypeId"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
