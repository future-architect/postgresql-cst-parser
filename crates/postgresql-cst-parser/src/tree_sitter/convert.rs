use std::collections::HashMap;

use cstree::{build::GreenNodeBuilder, syntax::SyntaxNode, traversal::WalkEvent};

use crate::{syntax_kind::SyntaxKind, NodeOrToken, PostgreSQLSyntax, ResolvedNode};

use super::Point;

type SequentialRange = cstree::text::TextRange; // Range representation by cstree (Sequential bytes)
type RowColumnRange = super::Range; // tree-sitter like range representation (Rows and Columns)

pub fn get_ts_tree_and_range_map(
    src: &str,
    root: &ResolvedNode,
) -> (ResolvedNode, HashMap<SequentialRange, RowColumnRange>) {
    let mut builder = GreenNodeBuilder::new();
    let mut row_column_ranges: Vec<RowColumnRange> = vec![];

    // Build new tree, and Collect row-column style Ranges
    {
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
        // process subtrees
        // These Nodes will be ignored:
        //   - Unneeded node
        //   - Nested node
        //   - Whitespace token
        //
        // Each Node in the tree:
        // 1. Add new Node (or Token) to New Tree
        // 2. Create tree-sitter compatible `Range`s based on the original text.
        walk_and_build(
            root,
            &new_line_indices,
            &mut builder,
            &mut row_column_ranges,
        );
        builder.finish_node();
    }

    // Get New tree
    let (tree, cache) = builder.finish();
    let new_root =
        SyntaxNode::new_root_with_resolver(tree, cache.unwrap().into_interner().unwrap());

    // Create a mapping between the TextRanges of nodes and tokens (in bytes) and the original text ranges (in rows and columns).
    let range_map = create_mapping(&new_root, row_column_ranges);

    (new_root, range_map)
}

fn get_row_column_range(node_or_token: &NodeOrToken, new_line_indices: &[usize]) -> RowColumnRange {
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

    let start_position = Point {
        row: before_start_new_line_count,
        column: usize::from(text_range.start())
            - match before_start_new_line_count {
                0 => 0,
                i => new_line_indices[i - 1] + 1,
            },
    };

    let end_position = Point {
        row: before_end_new_line_count,
        column: usize::from(text_range.end())
            - match before_end_new_line_count {
                0 => 0,
                i => new_line_indices[i - 1] + 1,
            },
    };

    RowColumnRange {
        start_byte: text_range.start().into(),
        end_byte: text_range.end().into(),
        start_position,
        end_position,
    }
}

fn create_mapping(
    root: &ResolvedNode,
    row_column_ranges: Vec<RowColumnRange>,
) -> HashMap<SequentialRange, RowColumnRange> {
    assert_eq!(
        root.descendants_with_tokens().count(),
        row_column_ranges.len()
    );

    let mut range_map: HashMap<SequentialRange, RowColumnRange> = HashMap::new();
    root.preorder_with_tokens()
        .filter(|e| matches!(e, WalkEvent::Enter(_)))
        .zip(row_column_ranges)
        .for_each(|(e, original_range)| match e {
            WalkEvent::Enter(node_or_token) => {
                let byte_range = node_or_token.text_range();
                range_map.insert(byte_range, original_range.clone());
            }
            _ => unreachable!(),
        });

    range_map
}

fn walk_and_build(
    node: &ResolvedNode,
    new_line_indices: &Vec<usize>,
    builder: &mut GreenNodeBuilder<'static, 'static, PostgreSQLSyntax>,
    row_column_ranges: &mut Vec<RowColumnRange>,
) {
    use cstree::util::NodeOrToken;

    let parent_kind = node.kind();
    let children = node.children_with_tokens();

    for child in children {
        match child {
            NodeOrToken::Node(child_node) => {
                match child_node.kind() {
                    child_kind @ (SyntaxKind::target_list
                    | SyntaxKind::from_list
                    | SyntaxKind::indirection
                    | SyntaxKind::expr_list
                    | SyntaxKind::func_arg_list
                    | SyntaxKind::when_clause_list
                    | SyntaxKind::group_by_list
                    | SyntaxKind::sortby_list
                    | SyntaxKind::qualified_name_list
                    | SyntaxKind::for_locking_items
                    | SyntaxKind::cte_list
                    | SyntaxKind::name_list
                    | SyntaxKind::set_clause_list
                    | SyntaxKind::set_target_list
                    | SyntaxKind::insert_column_list
                    | SyntaxKind::index_params
                    | SyntaxKind::values_clause
                    | SyntaxKind::TableFuncElementList) => {
                        if parent_kind == child_kind {
                            // [Node: Flatten]
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
                                child_node,
                                new_line_indices,
                                builder,
                                row_column_ranges,
                            );
                        } else {
                            // Node is target for flattening, but at the top level of the nest

                            row_column_ranges.push(get_row_column_range(
                                &NodeOrToken::Node(child_node),
                                new_line_indices,
                            ));

                            builder.start_node(child_node.kind());
                            walk_and_build(
                                child_node,
                                new_line_indices,
                                builder,
                                row_column_ranges,
                            );
                            builder.finish_node();
                        }
                    }

                    SyntaxKind::parse_toplevel
                    | SyntaxKind::stmtmulti
                    | SyntaxKind::toplevel_stmt
                    | SyntaxKind::stmt
                    | SyntaxKind::simple_select
                    | SyntaxKind::select_clause
                    | SyntaxKind::opt_select_limit
                    | SyntaxKind::opt_target_list
                    | SyntaxKind::opt_sort_clause
                    | SyntaxKind::select_limit => {
                        // [Node: Removal]
                        //
                        // Ignore current node, and continue building its children.
                        //
                        // (Old Tree)                                             (New Tree)
                        // *- parent_node            (ignore opt_target_list)     *- parent_node
                        //    +- opt_target_list    =========================>       +- child_1
                        //       +- child_1                                          +- child_2
                        //       +- child_1
                        //
                        walk_and_build(child_node, new_line_indices, builder, row_column_ranges);
                    }

                    // [Node: Default]
                    _ => {
                        row_column_ranges.push(get_row_column_range(
                            &NodeOrToken::Node(child_node),
                            new_line_indices,
                        ));
                        builder.start_node(child_node.kind());
                        walk_and_build(child_node, new_line_indices, builder, row_column_ranges);
                        builder.finish_node();
                    }
                }
            }
            NodeOrToken::Token(child_token) => {
                // [Token: Removal]
                // Note:
                //   This process will break the lossless property of the CST.
                //   `text()` for Nodes and `text_range()` for Nodes and Tokens will become incompatible with the original text.
                if child_token.kind() == SyntaxKind::Whitespace {
                    continue;
                }

                // [Token: Default]
                row_column_ranges.push(get_row_column_range(
                    &NodeOrToken::Token(child_token),
                    new_line_indices,
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

        let root = cst::parse(original).unwrap();
        let (new_root, _) = get_ts_tree_and_range_map(original, &root);

        let whitespace_removed: String = original.split_whitespace().collect();
        // Lossless property of the CST is broken.
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

        #[test]
        fn no_opt_select_limit() {
            let input = "select a from t for update limit 5 offset 5;";
            let root = cst::parse(input).unwrap();
            assert_exists(&root, SyntaxKind::opt_select_limit);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_not_exists(&new_root, SyntaxKind::opt_select_limit);
        }

        #[test]
        fn no_opt_sort_clause() {
            let input = "select a from t order by a desc limit 5;";
            let root = cst::parse(input).unwrap();
            assert_exists(&root, SyntaxKind::opt_sort_clause);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_not_exists(&new_root, SyntaxKind::opt_sort_clause);
        }

        #[test]
        fn no_select_limit() {
            let input = "select a from t limit 5;";
            let root = cst::parse(input).unwrap();
            assert_exists(&root, SyntaxKind::select_limit);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_not_exists(&new_root, SyntaxKind::select_limit);
        }
    }

    mod flatten {
        use crate::{
            cst,
            syntax_kind::SyntaxKind,
            tree_sitter::{
                assert_util::{
                    assert_direct_nested_kind, assert_no_direct_nested_kind, assert_node_count,
                },
                convert::get_ts_tree_and_range_map,
            },
        };

        #[test]
        fn no_nested_target_list() {
            let input = "select a,b,c;";

            let root = cst::parse(input).unwrap();
            assert_node_count(&root, SyntaxKind::target_list, 3);
            assert_direct_nested_kind(&root, SyntaxKind::target_list);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_node_count(&new_root, SyntaxKind::target_list, 1);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::target_list);
        }

        #[test]
        fn no_nested_stmtmulti() {
            let input = "select a,b,c;\nselect d,e from t;";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::stmtmulti);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::stmtmulti);
        }

        #[test]
        fn no_nested_from_list() {
            let input = "select * from t1, t2;";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::from_list);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::from_list);
        }

        #[test]
        fn no_nested_indirection() {
            let input =
                "select t.a, t.b.c, t1.*, a[1], a[4][5], a[2:5], a[3].b, a[3][4].b, a[3:5].b;";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::indirection);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::indirection);
        }

        #[test]
        fn no_nested_expr_list() {
            let input = "select a from t where a in (1,2,3);";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::expr_list);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::expr_list);
        }

        #[test]
        fn no_nested_func_arg_list() {
            let input = "select func(1, 2, func2(3, 4), 5);";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::func_arg_list);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::func_arg_list);
        }

        #[test]
        fn no_nested_when_clause_list() {
            let input = "select case when a then b when c then d when e then f else g end;";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::when_clause_list);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::when_clause_list);
        }

        #[test]
        fn no_nested_sortby_list() {
            let input = "select * from t order by a, b, c;";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::sortby_list);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::sortby_list);
        }

        #[test]
        fn no_nested_groupby_list() {
            let input = "select a, b, c from t group by a, b, c;";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::group_by_list);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::group_by_list);
        }

        #[test]
        fn no_nested_for_locking_items() {
            let input = "select * from t1, t2 for update of t1 for update of t2;";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::for_locking_items);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::for_locking_items);
        }

        #[test]
        fn no_nested_qualified_name_list() {
            let input = "select a from t for update of t.a, t.b;";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::qualified_name_list);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::qualified_name_list);
        }

        #[test]
        fn no_nested_cte_list() {
            let input = "with a as (select 1), b as (select 2) select * from a, b;";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::cte_list);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::cte_list);
        }

        #[test]
        fn no_nested_name_list() {
            let input = "with t (a, b) as (select 1) select * from t;";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::name_list);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::name_list);
        }

        #[test]
        fn no_nested_set_clause_list() {
            let input = "update t set a = 1, b = 2, c = 3;";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::set_clause_list);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::set_clause_list);
        }

        #[test]
        fn no_nested_set_target_list() {
            let input = "update t set (a, b, c) = (1, 2, 3) where id = 1;";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::set_target_list);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::set_target_list);
        }

        #[test]
        fn no_nested_insert_column_list() {
            let input = "insert into t (a, b, c) values (1, 2, 3);";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::insert_column_list);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::insert_column_list);
        }

        #[test]
        fn no_nested_index_params() {
            let input = "insert into t (a, b, c) values (1, 2, 3) on conflict (a, b) do nothing;";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::index_params);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::index_params);
        }

        #[test]
        fn no_nested_values_clause() {
            let input = "values (1,2,3), (4,5,6), (7,8,9);";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::values_clause);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::values_clause);
        }

        #[test]
        fn no_nested_table_func_element_list() {
            let input = "select * from unnest(a) as (x int, y text);";
            let root = cst::parse(input).unwrap();
            assert_direct_nested_kind(&root, SyntaxKind::TableFuncElementList);

            let (new_root, _) = get_ts_tree_and_range_map(input, &root);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::TableFuncElementList);
        }
    }
}
