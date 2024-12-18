use std::collections::HashMap;

use cstree::{build::GreenNodeBuilder, syntax::SyntaxNode, text::TextRange};

use crate::{syntax_kind::SyntaxKind, PostgreSQLSyntax, ResolvedNode};

use super::{traverse_pre_order, Range};

/// Converts the given CST into a node structure and hierarchy that closely matches what `tree-sitter-sql` produces.
pub fn convert_cst(src: &str, root: &ResolvedNode) -> (ResolvedNode, HashMap<TextRange, Range>) {
    let mut builder = GreenNodeBuilder::new();
    let mut range_vec = vec![];

    // Build `Root` node
    builder.start_node(SyntaxKind::Root);
    walk_and_build(src, &mut builder, &mut range_vec, root);
    builder.finish_node();

    let (tree, cache) = builder.finish();
    let resolved_node =
        SyntaxNode::new_root_with_resolver(tree, cache.unwrap().into_interner().unwrap());

    let mut range_map = HashMap::new();
    let mut ranges = range_vec.iter();
    traverse_pre_order(&resolved_node, |node_or_token| {
        if let Some(original_range) = ranges.next() {
            // clone?
            range_map.insert(node_or_token.text_range(), original_range.clone());
        } else {
            unreachable!()
        }
    });

    (resolved_node, range_map)
}

/// Traverse the CST and rewrite certain nodes
/// e.g. flatten list node, remove option node
fn walk_and_build(
    input: &str,
    builder: &mut GreenNodeBuilder<'static, 'static, PostgreSQLSyntax>,
    range_vec: &mut Vec<Range>,
    node: &ResolvedNode,
) {
    use cstree::util::NodeOrToken;

    let new_line_indices: Vec<_> = input
        .char_indices()
        .filter(|&(_, c)| c == '\n')
        .map(|(i, _)| i)
        .collect();

    let text_range = node.text_range();

    // text_range の初期部分が、改行のどの部分に現れるか
    let before_start_new_line_count =
        match new_line_indices.binary_search(&text_range.start().into()) {
            Ok(i) => i,
            Err(i) => i,
        };

    let before_end_new_line_count = match new_line_indices.binary_search(&text_range.end().into()) {
        Ok(i) => i,
        Err(i) => i,
    };

    range_vec.push(Range {
        start_row: before_start_new_line_count,
        start_col: usize::from(node.text_range().start())
            - match before_start_new_line_count {
                0 => 0,
                // ひとつ前のインデックス（直前の改行文字）+1
                i => new_line_indices[i - 1] + 1, // +1 は改行文字分？
            },
        end_row: before_end_new_line_count,
        end_col: usize::from(node.text_range().end())
            - 1
            - match before_end_new_line_count {
                0 => 0,
                i => new_line_indices[i - 1],
            },
    });

    let parent_kind = node.kind();
    let children = node.children_with_tokens();

    for child in children {
        match child {
            NodeOrToken::Node(n) => {
                match n.kind() {
                    child_kind @ (SyntaxKind::stmtmulti
                    | SyntaxKind::target_list
                    | SyntaxKind::from_list) => {
                        if parent_kind == child_kind {
                            // [Flatten]
                            //
                            // This patten does not construct node.
                            //
                            // * target_list (parent)   <- 1. A node that passed as an argument of this function.
                            //   +- target_el           <- 2. This node (or token) was already consumed in previous loop.
                            //   +- target_list (child) <- 3. This is the nested node (parent is the same syntax kind).  Just ignore this node, and continue building its children.
                            //     +- target_el
                            //     +- ...
                            //
                            walk_and_build(input, builder, range_vec, n);
                        } else {
                            // Node is target for flattening, but at the top level of the nest
                            builder.start_node(n.kind());
                            walk_and_build(input, builder, range_vec, n);
                            builder.finish_node();
                        }
                    }

                    // SyntaxKind::parse_toplevel
                    // | SyntaxKind::stmtmulti
                    // | SyntaxKind::toplevel_stmt
                    // | SyntaxKind::stmt
                    // | SyntaxKind::select_no_parens
                    // | SyntaxKind::simple_select
                    // |
                    SyntaxKind::opt_target_list => {
                        // [Removal]
                        //
                        // Ignore current node, and continue building its children.
                        //
                        // (Old Tree)                                             (New Tree)
                        // *- parent_node            (ignore opt_target_list)     *- parent_node
                        //    +- opt_target_list    =========================>       +- child_1
                        //       +- child_1                                          +- child_2
                        //       +- child_1
                        //
                        walk_and_build(input, builder, range_vec, n);
                    }

                    // Default pattern
                    _ => {
                        builder.start_node(n.kind());
                        walk_and_build(input, builder, range_vec, n);
                        builder.finish_node();
                    }
                }
            }
            NodeOrToken::Token(t) => {
                // Remove Whitespace Token
                // Note:
                //   This process will break the lossless property of the CST.
                //   `text()` for Nodes and `text_range()` for Nodes and Tokens will become incompatible with the original text.
                if t.kind() == SyntaxKind::Whitespace {
                    continue;
                }

                builder.token(t.kind(), t.text());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cst, tree_sitter::convert::convert_cst};

    #[test]
    fn whitespace_is_removed() {
        let original = r#"
SELECT
	1 as X
,	2
,	3
FROM
	A
,	B"#;

        let root = cst::parse(&original).unwrap();
        let new_root = convert_cst(&original, &root);

        let whitespace_removed: String = original.split_whitespace().collect();
        dbg!(&whitespace_removed);
        assert_eq!(new_root.text(), whitespace_removed.as_str());
    }

    mod removal {
        use crate::{
            cst,
            syntax_kind::SyntaxKind,
            tree_sitter::{
                assert_util::{assert_exists, assert_not_exists},
                convert::convert_cst,
            },
        };

        #[test]
        fn no_opt_target_list() {
            let input = "select a,b,c;";
            let root = cst::parse(input).unwrap();
            assert_exists(&root, SyntaxKind::opt_target_list);

            let new_root = convert_cst(input, &root);
            assert_not_exists(&new_root, SyntaxKind::opt_target_list);
        }
    }

    mod flatten {
        use crate::{
            cst,
            syntax_kind::SyntaxKind,
            tree_sitter::{
                assert_util::{assert_no_direct_nested_kind, assert_node_count},
                convert::convert_cst,
            },
        };

        #[test]
        fn no_nested_target_list() {
            let input = "select a,b,c;";

            let root = cst::parse(input).unwrap();
            assert_node_count(&root, SyntaxKind::target_list, 3);

            let new_root = convert_cst(input, &root);
            assert_node_count(&new_root, SyntaxKind::target_list, 1);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::target_list);
        }

        #[test]
        fn no_nested_stmtmulti() {
            let input = "select a,b,c;\nselect d,e from t;";
            let root = cst::parse(input).unwrap();
            let new_root = convert_cst(&input, &root);

            assert_no_direct_nested_kind(&new_root, SyntaxKind::stmtmulti);
        }

        #[test]
        fn no_nested_from_list() {
            let input = "select * from t1, t2;";
            let root = cst::parse(input).unwrap();
            let new_root = convert_cst(input, &root);

            assert_no_direct_nested_kind(&new_root, SyntaxKind::from_list);
        }
    }
}
