use crate::ast;

fn add(tab: &str) -> String {
	return format!("   {}", tab)
}

fn encode_list<'a, T>(arr: &Vec<T>, func: impl Fn(&T) -> String) -> String {
	arr.iter().map(func).collect::<Vec<String>>().join(", ")
}

fn encode_statment<'a>(statment: &ast::Statment<'a>, tab: &str) -> String {
	return match statment {
		ast::Statment::Assign (name, node) => format!("let {} = {}", name, encode(node, tab)),
		ast::Statment::Effect (node) => encode(node, tab),
		ast::Statment::Output (node) => format!("return {}", encode(node, tab)),
	};
}

fn encode_statment_block<'a>(stm_block: &ast::StmBlock<'a>, tab: &str) -> String {
	return format!("\n{}", stm_block.statments.iter().map(|stm|
		format!("{}{}\n", tab, encode_statment(stm, tab))).collect::<Vec<String>>().join("")
	);
}

fn encode<'a>(node: &'a ast::Node<'a> , tab: &str) -> String {
	match node {
		ast::Node::FuncMake (a) => format!("([{}]) {}",
			encode_list(
				& a.params,
				|arg| format!("{} {}", encode(&arg.kind, tab), arg.name)
			),
			encode_statment_block( &a.body, &add(tab) ),
		),
		ast::Node::FuncCall (a) => format!("{}({})",
			encode(a.func, tab),
			encode_list(&a.args, |arg| encode(&arg, tab))
		),
		// ast::Node::StmBlock (a) => encode_statment_block(a, &add(tab)),
		ast::Node::Variable (a) => format!("{}", a.name),
		ast::Node::NumValue (a) => format!("{}", a.value)
	}
}

pub fn generate<'a>(node:  &'a ast::Node<'a>) -> String {
	return encode(node, "")
}