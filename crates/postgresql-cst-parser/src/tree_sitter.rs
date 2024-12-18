#[cfg(test)]
mod assert_util;

mod convert;
pub use convert::convert_cst;

use std::{collections::HashMap, fmt::Display, rc::Rc};

use cstree::text::TextRange;

use crate::{syntax_kind::SyntaxKind, NodeOrToken, ResolvedNode};

impl Display for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
    input: &'a str,
    range_map: Rc<HashMap<TextRange, Range>>,
    pub node_or_token: NodeOrToken<'a>,
}

#[derive(Debug, Clone)]
pub struct TreeCursor<'a> {
    pub input: &'a str,
    range_map: Rc<HashMap<TextRange, Range>>,
    node_or_token: NodeOrToken<'a>,
}

#[derive(Debug, Clone)]
pub struct Range {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[({}, {})-({}, {})]",
            self.start_row, self.start_col, self.end_row, self.end_col
        )
    }
}

// fn is_flatten_all(node_or_token: NodeOrToken) -> bool {
//     matches!(
//         node_or_token.kind(),
//         SyntaxKind::parse_toplevel
//             | SyntaxKind::stmtmulti
//             | SyntaxKind::toplevel_stmt
//             | SyntaxKind::stmt
//             | SyntaxKind::select_clause
//             | SyntaxKind::select_with_parens
//             | SyntaxKind::select_no_parens
//             | SyntaxKind::simple_select
//             | SyntaxKind::opt_target_list
//             // | SyntaxKind::relation_expr
//             // | SyntaxKind::extended_relation_expr
//             // | SyntaxKind::qualified_name
//             // | SyntaxKind::indirection
//             // | SyntaxKind::indirection_el
//             // | SyntaxKind::table_ref
//             | SyntaxKind::alias_clause
//             | SyntaxKind::opt_alias_clause
//     )
// }

// fn is_flatten_except_top(node_or_token: NodeOrToken) -> bool {
//     matches!(
//         node_or_token.kind(),
//         SyntaxKind::target_list | SyntaxKind::from_list
//     ) && node_or_token.parent().unwrap().kind() == node_or_token.kind()
// }

// fn is_flatten(node_or_token: NodeOrToken) -> bool {
//     is_flatten_all(node_or_token) || is_flatten_except_top(node_or_token)
// }

// fn is_skip(kind: SyntaxKind) -> bool {
//     matches!(kind, SyntaxKind::Whitespace)
// }

impl<'a> Node<'a> {
    pub fn walk(&self) -> TreeCursor<'a> {
        unimplemented!()
    }

    pub fn kind(&self) -> SyntaxKind {
        self.node_or_token.kind()
    }

    pub fn range(&self) -> Range {
        // dbg!(self.node_or_token.text_range(), &self.range_map);
        self.range_map
            .get(&self.node_or_token.text_range())
            .cloned()
            .unwrap()
    }

    pub fn text(&self) -> &'a str {
        // self.node_or_token
        //     .as_token()
        //     .map(|t| t.text())
        //     .unwrap_or_default()
        let start = self.node_or_token.text_range().start().into();
        let end = self.node_or_token.text_range().end().into();
        &self.input[start..end]
    }

    pub fn child_count(&self) -> usize {
        if let Some(node) = self.node_or_token.as_node() {
            return node.children_with_tokens().count();
        }
        0
    }

    pub fn next_sibling(&self) -> Option<Node<'a>> {
        self.node_or_token
            .next_sibling_or_token()
            .map(|sibling| Node {
                input: self.input,
                range_map: Rc::clone(&self.range_map),
                node_or_token: sibling,
            })
    }

    pub fn is_comment(&self) -> bool {
        matches!(self.kind(), SyntaxKind::C_COMMENT | SyntaxKind::SQL_COMMENT)
    }
}

impl<'a> From<Node<'a>> for TreeCursor<'a> {
    fn from(value: Node<'a>) -> Self {
        Self {
            input: value.input,
            range_map: value.range_map,
            node_or_token: value.node_or_token,
        }
    }
}

impl<'a> TreeCursor<'a> {
    pub fn node(&self) -> Node<'a> {
        Node {
            input: self.input,
            range_map: Rc::clone(&self.range_map),
            node_or_token: self.node_or_token,
        }
    }

    pub fn goto_first_child(&mut self) -> bool {
        if let Some(current_node) = self.node_or_token.as_node() {
            if let Some(child) = current_node.first_child_or_token() {
                self.node_or_token = child;
                return true;
            }
        }
        false
    }

    pub fn goto_parent(&mut self) -> bool {
        if let Some(parent) = self.node_or_token.parent() {
            self.node_or_token = NodeOrToken::Node(parent);

            return true;
        }

        false
    }

    pub fn goto_next_sibling(&mut self) -> bool {
        if let Some(sibling) = self.node_or_token.next_sibling_or_token() {
            self.node_or_token = sibling;
            return true;
        }
        false
    }

    ///
    /// These methods are unused in uroborosql-fmt
    ///
    // pub fn field_id(&self)-> Option<u16> {
    //     unimplemented!()
    // }

    // pub fn field_name(&self)-> Option<&'static str> {
    //     unimplemented!()
    // }

    // pub fn goto_first_child_for_byte(&mut self, index: usize) -> Option<usize> {
    //     unimplemented!()
    // }

    // pub fn goto_first_child_for_point(&mut self, point: Point) -> Option<usize> {
    //  unimplemented!()
    // }

    // pub fn reset(&mut self, node: Node<'a>) {
    //     unimplemented!()
    // }

    pub fn goto_direct_prev_sibling(&mut self) -> bool {
        if let Some(prev) = self.node_or_token.prev_sibling_or_token() {
            self.node_or_token = prev;
            true
        } else {
            false
        }
    }

    pub fn is_comment(&self) -> bool {
        matches!(
            self.node_or_token.kind(),
            SyntaxKind::C_COMMENT | SyntaxKind::SQL_COMMENT
        )
    }
}

pub fn as_tree_sitter_cursor<'a>(input: &'a str, node: &'a ResolvedNode) -> TreeCursor<'a> {
    let mut range_map = HashMap::new();

    let new_line_indices: Vec<_> = input
        .char_indices()
        .filter(|&(_, c)| c == '\n')
        .map(|(i, _)| i)
        .collect();

    traverse_pre_order(node, |node_or_token| {
        let text_range = node_or_token.text_range();

        let before_start_new_line_count =
            match new_line_indices.binary_search(&text_range.start().into()) {
                Ok(i) => i,
                Err(i) => i,
            };

        let before_end_new_line_count =
            match new_line_indices.binary_search(&text_range.end().into()) {
                Ok(i) => i,
                Err(i) => i,
            };

        range_map.insert(
            node_or_token.text_range(),
            Range {
                start_row: before_start_new_line_count,
                start_col: usize::from(node_or_token.text_range().start())
                    - match before_start_new_line_count {
                        0 => 0,
                        i => new_line_indices[i - 1] + 1,
                    },
                end_row: before_end_new_line_count,
                end_col: usize::from(node_or_token.text_range().end())
                    - 1
                    - match before_end_new_line_count {
                        0 => 0,
                        i => new_line_indices[i - 1],
                    },
            },
        );
    });

    TreeCursor {
        input,
        range_map: Rc::new(range_map),
        node_or_token: NodeOrToken::Node(node),
    }
}

fn traverse_pre_order<F: FnMut(NodeOrToken)>(node: &ResolvedNode, mut f: F) {
    let mut node_or_token = NodeOrToken::Node(node);

    loop {
        f(node_or_token);

        if let Some(node) = node_or_token.as_node() {
            if let Some(child) = node.first_child_or_token() {
                node_or_token = child;
                continue;
            }
        }

        if let Some(sibling) = node_or_token.next_sibling_or_token() {
            node_or_token = sibling;
        } else {
            loop {
                if let Some(parent) = node_or_token.parent() {
                    node_or_token = NodeOrToken::Node(parent);
                } else {
                    return;
                }

                if let Some(sibling) = node_or_token.next_sibling_or_token() {
                    node_or_token = sibling;
                    break;
                }
            }
        }
    }
}

pub fn dump_as_tree_sitter_like(input: &str, node: &ResolvedNode) {
    let mut cursor = as_tree_sitter_cursor(input, node);

    let mut depth = 0;
    loop {
        // dbg!(cursor.node().kind(), cursor.node().text(), depth);

        if cursor.goto_first_child() {
            depth += 1;
        } else if cursor.goto_next_sibling() {
        } else {
            loop {
                if !cursor.goto_parent() {
                    return;
                }

                depth -= 1;
                if cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cst, parse,
        syntax_kind::SyntaxKind,
        tree_sitter::{as_tree_sitter_cursor, convert_cst, dump_as_tree_sitter_like, TreeCursor},
        ParseError,
    };

    #[test]
    fn test() -> Result<(), ParseError> {
        let input = r#"
SELECT
	1 as X
,	2
,	3
FROM
	A
,	B"#;
        // dbg!(input);
        let node = cst::parse(input)?;
        // dbg!(&node);

        dump_as_tree_sitter_like(input, &node);

        Ok(())
    }

    #[test]
    fn tree_sitter_like_traverse() {
        const UNIT: usize = 2;

        fn visit(cursor: &mut TreeCursor, depth: usize, src: &str) {
            (0..(depth * UNIT)).for_each(|_| print!(" "));

            print!("{}", cursor.node().kind());

            if cursor.node().child_count() == 0 {
                // print!(" \"{}\"", cursor.node().utf8_text(src.as_bytes()).unwrap());
                print!(" \"{}\"", cursor.node().text().escape_default());
            }
            println!(
                // " [{}-{}]",
                // cursor.node().start_position(),
                // cursor.node().end_position()
                " {}",
                cursor.node().range()
            );

            // 子供を走査
            if cursor.goto_first_child() {
                visit(cursor, depth + 1, src);
                while cursor.goto_next_sibling() {
                    visit(cursor, depth + 1, src);
                }
                cursor.goto_parent();
            }
        }

        let src = r#"
-- comment
SELECT
	1 as X
,	2 -- comment
,	3
FROM
	A
,	B"#;

        let node = parse(&src).unwrap();
        let node = convert_cst(&node);
        let mut cursor = as_tree_sitter_cursor(src, &node);

        visit(&mut cursor, 0, &src);
    }

    #[test]
    fn goto_first_child_from_node() {
        let src = "select a, b, c from tbl;";
        let root = convert_cst(&parse(&src).unwrap());
        let first_select = root
            .descendants()
            .find(|x| x.kind() == SyntaxKind::simple_select)
            .unwrap();

        let mut cursor = as_tree_sitter_cursor(src, &first_select);
        assert_eq!(cursor.node().kind(), SyntaxKind::simple_select);

        assert!(cursor.goto_first_child());
        assert_eq!(cursor.node().kind(), SyntaxKind::SELECT);
    }

    #[test]
    fn goto_first_child_from_token() {
        let src = "select a, b, c from tbl;";
        let root = convert_cst(&parse(&src).unwrap());
        let column_id_node = root
            .descendants()
            .find(|x| x.kind() == SyntaxKind::ColId)
            .unwrap();

        let mut cursor = as_tree_sitter_cursor(src, &column_id_node);
        cursor.goto_first_child();
        assert_eq!(cursor.node().kind(), SyntaxKind::IDENT);

        assert!(!cursor.goto_first_child());
        assert_eq!(cursor.node().kind(), SyntaxKind::IDENT);
    }

    #[test]
    fn goto_parent_from_root() {
        let src = "select a, b, c from tbl;";
        let root = convert_cst(&parse(&src).unwrap());

        let mut cursor = as_tree_sitter_cursor(src, &root);
        assert_eq!(cursor.node().kind(), SyntaxKind::Root);

        assert!(!cursor.goto_parent());
        assert_eq!(cursor.node().kind(), SyntaxKind::Root);
    }

    #[test]
    fn goto_parent_from_node() {
        let src = "select a, b, c from tbl;";
        let root = convert_cst(&parse(&src).unwrap());

        let target_element = root
            .descendants()
            .find(|x| x.kind() == SyntaxKind::target_el)
            .unwrap();
        let mut cursor = as_tree_sitter_cursor(src, &target_element);
        assert_eq!(cursor.node().kind(), SyntaxKind::target_el);

        assert!(cursor.goto_parent());
        assert_eq!(cursor.node().kind(), SyntaxKind::target_list);
    }

    #[test]
    fn goto_parent_from_token() {
        let src = "select a, b, c from tbl;";
        let root = convert_cst(&parse(&src).unwrap());

        let column_id_node = root
            .descendants()
            .find(|x| x.kind() == SyntaxKind::ColId)
            .unwrap();
        let mut cursor = as_tree_sitter_cursor(src, &column_id_node);

        cursor.goto_first_child();
        assert_eq!(cursor.node().kind(), SyntaxKind::IDENT);

        assert!(cursor.goto_parent());
        assert_eq!(cursor.node().kind(), SyntaxKind::ColId);
    }

    #[test]
    fn goto_next_sibling() {
        let src = "select a,b,c from tbl;";
        let root = convert_cst(&parse(&src).unwrap());

        let target_element = root
            .descendants()
            .find(|x| x.kind() == SyntaxKind::target_el)
            .unwrap();
        let mut cursor = as_tree_sitter_cursor(src, &target_element);
        //
        // - target_list
        //   - target_el (1)
        //   - Comma ","
        //   - target_el (2)
        //   - Comma ","
        //   - target_el (3)
        //

        // 1
        assert_eq!(cursor.node().kind(), SyntaxKind::target_el);

        assert!(cursor.goto_next_sibling());
        assert_eq!(cursor.node().kind(), SyntaxKind::Comma);

        // 2
        assert!(cursor.goto_next_sibling());
        assert_eq!(cursor.node().kind(), SyntaxKind::target_el);

        assert!(cursor.goto_next_sibling());
        assert_eq!(cursor.node().kind(), SyntaxKind::Comma);

        // 3
        assert!(cursor.goto_next_sibling());
        assert_eq!(cursor.node().kind(), SyntaxKind::target_el);

        // No more siblings
        assert!(!cursor.goto_next_sibling());
        assert_eq!(cursor.node().kind(), SyntaxKind::target_el);
    }
}
