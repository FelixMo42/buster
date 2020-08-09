use crate::ast;
use crate::scope;
fn add(tab: &str) -> String {
	return format!("   {}", tab)
}

fn str_map<'a, T>(arr: &Vec<T>, func: impl Fn(&T) -> String) -> Vec<String> {
	return arr.iter().map(func).collect::<Vec<String>>();
}

fn encode_statment<'a>(statment: &ast::Statment<'a>, tab: &str) -> String {
	return match statment {
		ast::Statment::Assign (name, node) => format!("let {} = {}", name, encode(node, tab)),
		ast::Statment::Effect (node) => encode(node, tab),
		ast::Statment::Output (node) => format!("return {}", encode(node, tab)),
	};
}

fn encode_statment_block<'a>(stm_block: &ast::StmBlock<'a>, tab: &str) -> String {
	str_map(&stm_block.statments, |stm| format!("{}{}\n", tab, encode_statment(stm, tab))).join("")
}

fn encode_kind<'a>(kind: &'a scope::Kind , _tab: &str) -> String {
	return format!("{}", kind.base);
}

fn encode<'a>(node: &'a ast::Node<'a> , tab: &str) -> String {
	match node {
		ast::Node::FuncMake (a) => format!("([{}]\n{})",
			str_map(
				& a.params,
				|arg| format!("{} {}", encode_kind(&arg.kind, tab), arg.name)
			).join(", "),
			encode_statment_block( &a.body, &add(tab) ),
		),
		ast::Node::FuncCall (a) => format!("{}({})",
			encode(a.func, tab),
			str_map(&a.args, |arg| encode(&arg, tab)).join(", ")
		),
		// ast::Node::StmBlock (a) => encode_statment_block(a, &add(tab)),
		ast::Node::Variable (a) => format!("{}", a.name),
		ast::Node::NumValue (a) => format!("{}", a.value)
	}
}

pub fn generate<'a>(node:  &'a ast::Node<'a>) -> String {
	return encode(node, "")
}