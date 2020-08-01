use crate::ast;

fn add(tab: &str) -> String {
	return format!("   {}", tab)
}

fn encode_list<'a, T>(arr: &Vec<T>, func: impl Fn(&T) -> String) -> String {
	arr.iter().map(func).collect::<Vec<String>>().join(", ")
}

fn encode_statment<'a>(statment: &ast::Statment<'a>, tab: &str) -> String {
	match statment {
		ast::Statment::Assign (name, node) => format!("let {} = {}", name, encode(node, tab)),
		ast::Statment::Effect (node) => encode(node, tab),
		ast::Statment::Output (node) => format!("return {}", encode(node, tab)),
	}
}

fn encode_statment_block<'a>(stm_block: &ast::StmBlock<'a>, tab: &str) -> String {
	format!("\n{}", stm_block.statments.iter().map(|stm|
		format!("{}{}\n", tab, encode_statment(stm, tab))).collect::<Vec<String>>().join("")
	)
}

fn encode_operator_call<'a>(operator: &str, args: &Vec<ast::Node>, tab: &str) -> String {
	return format!("({a} {operator} {b})",
		a = encode(&args[0], tab),
		b = encode(&args[1], tab),
		operator = operator
	);
}

fn encode_lang_function_call<'a>(name: &str, args: &Vec<ast::Node>, tab: &str) -> String {
	return encode_operator_call(name, args, tab);
}

fn encode_prog_function_call<'a>(call: &ast::FuncCall, tab: &str) -> String {
	format!("{}({})",
		encode(call.func, tab),
		encode_list(&call.args, |arg| encode(&arg, tab))
	)
}

fn encode_function_call<'a>(call: &ast::FuncCall, tab: &str) -> String {
	let func_type = call.func.kind();

	let add_lang_func = String::from("+");

	match &func_type.lang {
		Some (add_lang_func) => encode_lang_function_call("+", &call.args, tab),
		Some (_) => encode_prog_function_call(call, tab),
		None => encode_prog_function_call(call, tab)
	}
}

fn encode_function_make<'a>(func: &ast::FuncMake, tab: &str) -> String {
	return format!("(({params}) => {{{body}}})",
		params = encode_list(&func.params, |param| format!("{}", param.name)),
		body   = encode_statment_block(&func.body, &add(tab) ),
	);
}

fn encode<'a>(node: &'a ast::Node<'a> , tab: &str) -> String {
	match node {
		ast::Node::FuncMake (node) => encode_function_make(node, tab),
		ast::Node::FuncCall (node) => encode_function_call(node, tab),
		ast::Node::Variable (node) => format!("{}", node.name),
		ast::Node::NumValue (node) => format!("{}", node.value)
	}
}

pub fn generate<'a>(node:  &'a ast::Node<'a>) -> String {
	return encode(node, "")
}