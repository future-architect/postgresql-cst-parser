use std::collections::HashMap;

use cstree::{build::GreenNodeBuilder, syntax::SyntaxNode};

use crate::{syntax_kind::SyntaxKind, NodeOrToken, PostgreSQLSyntax, ResolvedNode};

use super::traverse_pre_order;

type SequentialRange = cstree::text::TextRange; // Range representation by cstree
type RowColumnRange = super::Range; // tree-sitter like range representation

pub fn get_ts_tree_and_range_map(
    src: &str,
    root: &ResolvedNode,
) -> (ResolvedNode, HashMap<SequentialRange, RowColumnRange>) {
    let mut builder = GreenNodeBuilder::new();
    let mut row_column_ranges: Vec<RowColumnRange> = vec![];

    let new_line_indices: Vec<_> = src
        .char_indices()
        .filter(|&(_, c)| c == '\n')
        .map(|(i, _)| i)
        .collect();

    row_column_ranges.push(get_row_column_range(
        &NodeOrToken::Node(root),
        &new_line_indices,
    ));

    builder.start_node(SyntaxKind::Root);
    // process subtrees.
    // These Nodes will be ignored:
    //   - unneeded node
    //   - nested node
    //   - Whitespace token
    //
    // Each Node in the tree:
    // 1. Add new Node (or Token) to New Tree
    // 2. Create tree-sitter compatible `Range`s based on the original text.
    walk_and_build(
        src,
        &new_line_indices,
        &mut builder,
        &mut row_column_ranges,
        &root,
    );
    builder.finish_node();

    let (tree, cache) = builder.finish();
    let new_root =
        SyntaxNode::new_root_with_resolver(tree, cache.unwrap().into_interner().unwrap());

    assert_eq!(
        new_root.descendants_with_tokens().count(),
        row_column_ranges.len()
    );

    let mut range_map: HashMap<SequentialRange, RowColumnRange> = HashMap::new();
    let mut range_iter = row_column_ranges.iter();
    traverse_pre_order(&new_root, |node_or_token| {
        if let Some(original_range) = range_iter.next() {
            let byte_range = node_or_token.text_range();
            range_map.insert(byte_range, original_range.clone());
        } else {
            unreachable!();
        }
    });

    assert!(range_iter.next().is_none());

    (new_root, range_map)
}

fn get_row_column_range(
    node_or_token: &NodeOrToken,
    new_line_indices: &Vec<usize>,
) -> RowColumnRange {
    let text_range: SequentialRange = node_or_token.text_range();

    let before_start_new_line_count =
        match new_line_indices.binary_search(&text_range.start().into()) {
            Ok(i) => i,
            Err(i) => i,
        };

    let before_end_new_line_count = match new_line_indices.binary_search(&text_range.end().into()) {
        Ok(i) => i,
        Err(i) => i,
    };

    RowColumnRange {
        start_row: before_start_new_line_count,
        start_col: usize::from(text_range.start())
            - match before_start_new_line_count {
                0 => 0,
                i => new_line_indices[i - 1] + 1,
            },
        end_row: before_end_new_line_count,
        end_col: usize::from(text_range.end())
            - 1
            - match before_end_new_line_count {
                0 => 0,
                i => new_line_indices[i - 1],
            },
    }
}

/// Traverse the CST and rewrite certain nodes
/// e.g. flatten list node, remove option node
fn walk_and_build(
    input: &str,
    new_line_indices: &Vec<usize>,
    builder: &mut GreenNodeBuilder<'static, 'static, PostgreSQLSyntax>,
    row_column_ranges: &mut Vec<RowColumnRange>,
    node: &ResolvedNode,
) {
    use cstree::util::NodeOrToken;

    let parent_kind = node.kind();
    let children = node.children_with_tokens();

    for child in children {
        match child {
            NodeOrToken::Node(child_node) => {
                match child_node.kind() {
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
                            walk_and_build(
                                input,
                                new_line_indices,
                                builder,
                                row_column_ranges,
                                child_node,
                            );
                        } else {
                            // Node is target for flattening, but at the top level of the nest
                            
                            row_column_ranges.push(get_row_column_range(
                                &NodeOrToken::Node(&child_node),
                                &new_line_indices,
                            ));

                            builder.start_node(child_node.kind());
                            walk_and_build(
                                input,
                                new_line_indices,
                                builder,
                                row_column_ranges,
                                child_node,
                            );
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
                        walk_and_build(
                            input,
                            new_line_indices,
                            builder,
                            row_column_ranges,
                            child_node,
                        );
                    }

                    // Default pattern
                    _ => {
                        row_column_ranges.push(get_row_column_range(
                            &NodeOrToken::Node(&child_node),
                            &new_line_indices,
                        ));
                        builder.start_node(child_node.kind());
                        walk_and_build(
                            input,
                            new_line_indices,
                            builder,
                            row_column_ranges,
                            child_node,
                        );
                        builder.finish_node();
                    }
                }
            }
            NodeOrToken::Token(child_token) => {
                // Remove Whitespace Token
                // Note:
                //   This process will break the lossless property of the CST.
                //   `text()` for Nodes and `text_range()` for Nodes and Tokens will become incompatible with the original text.
                if child_token.kind() == SyntaxKind::Whitespace {
                    continue;
                }

                row_column_ranges.push(get_row_column_range(
                    &NodeOrToken::Token(child_token),
                    &new_line_indices,
                ));
                builder.token(child_token.kind(), child_token.text());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cst, tree_sitter::convert::get_ts_tree_and_range_map};

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
        let (new_root, _) = get_ts_tree_and_range_map(&original, &root);

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
                convert::get_ts_tree_and_range_map,
            },
        };

        #[test]
        fn no_opt_target_list() {
            let input = "select a,b,c;";
            let root = cst::parse(input).unwrap();
            assert_exists(&root, SyntaxKind::opt_target_list);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_not_exists(&new_root, SyntaxKind::opt_target_list);
        }
    }

    mod flatten {
        use crate::{
            cst,
            syntax_kind::SyntaxKind,
            tree_sitter::{
                assert_util::{assert_no_direct_nested_kind, assert_node_count},
                convert::get_ts_tree_and_range_map,
            },
        };

        #[test]
        fn no_nested_target_list() {
            let input = "select a,b,c;";

            let root = cst::parse(input).unwrap();
            assert_node_count(&root, SyntaxKind::target_list, 3);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_node_count(&new_root, SyntaxKind::target_list, 1);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::target_list);
        }

        #[test]
        fn no_nested_stmtmulti() {
            let input = "select a,b,c;\nselect d,e from t;";
            let root = cst::parse(input).unwrap();
            let (new_root, _) = get_ts_tree_and_range_map(input, &root);

            assert_no_direct_nested_kind(&new_root, SyntaxKind::stmtmulti);
        }

        #[test]
        fn no_nested_from_list() {
            let input = "select * from t1, t2;";
            let root = cst::parse(input).unwrap();
            let (new_root, _) = get_ts_tree_and_range_map(&input, &root);

            assert_no_direct_nested_kind(&new_root, SyntaxKind::from_list);
        }
    }
}
