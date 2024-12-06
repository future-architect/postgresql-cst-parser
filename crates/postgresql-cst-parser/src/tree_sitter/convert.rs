use cstree::{build::GreenNodeBuilder, syntax::SyntaxNode};

use crate::{syntax_kind::SyntaxKind, PostgreSQLSyntax, ResolvedNode};

/// Converts the given CST into a node structure and hierarchy that closely matches what `tree-sitter-sql` produces.
pub fn convert_cst(root: &ResolvedNode) -> ResolvedNode {
    let mut builder = GreenNodeBuilder::new();

    // Build `Root` node
    builder.start_node(SyntaxKind::Root);
    walk_and_build(&mut builder, root);
    builder.finish_node();

    let (tree, cache) = builder.finish();

    SyntaxNode::new_root_with_resolver(tree, cache.unwrap().into_interner().unwrap())
}

/// CST を走査し、いくつかの Node を書き換える
/// e.g. flatten list node, remove option node
fn walk_and_build(
    builder: &mut GreenNodeBuilder<'static, 'static, PostgreSQLSyntax>,
    node: &ResolvedNode,
) {
    use cstree::util::NodeOrToken;
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
                            walk_and_build(builder, n);
                        } else {
                            // フラット化の対象だが、ネストのトップにあるノード
                            builder.start_node(n.kind());
                            walk_and_build(builder, n);
                            builder.finish_node();
                        }
                    }

                    SyntaxKind::opt_target_list => {
                        // [Removal]
                        //
                        // Ignore current node, and continue building its children.
                        //
                        // (Old Tree)                                                   (New Tree)
                        // *- parent_node            (ignore opt_target_list)     *- parent_node
                        //    +- opt_target_list    =========================>       +- child_1
                        //       +- child_1                                          +- child_2
                        //       +- child_1
                        //
                        walk_and_build(builder, n);
                    }

                    // Default pattern
                    _ => {
                        builder.start_node(n.kind());
                        walk_and_build(builder, n);
                        builder.finish_node();
                    }
                }
            }
            NodeOrToken::Token(t) => {
                builder.token(t.kind(), t.text());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cst, tree_sitter::convert::convert_cst};

    #[test]
    ///  Assert that the CST is not broken by the conversion.
    fn restored_texts_are_equal() {
        let input = r#"
SELECT
	1 as X
,	2
,	3
FROM
	A
,	B"#;

        let root = cst::parse(input).unwrap();
        let new_root = convert_cst(&root);

        //  format!("{ResolvedNode}") returns original input str.
        assert_eq!(format!("{root}"), format!("{new_root}"));
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

            let new_root = convert_cst(&root);
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

            let new_root = convert_cst(&root);
            assert_node_count(&new_root, SyntaxKind::target_list, 1);
            assert_no_direct_nested_kind(&new_root, SyntaxKind::target_list);
        }

        #[test]
        fn no_nested_stmtmulti() {
            let input = "select a,b,c;\nselect d,e from t;";
            let root = cst::parse(input).unwrap();
            let new_root = convert_cst(&root);

            assert_no_direct_nested_kind(&new_root, SyntaxKind::stmtmulti);
        }

        #[test]
        fn no_nested_from_list() {
            let input = "select * from t1, t2;";
            let root = cst::parse(input).unwrap();
            let new_root = convert_cst(&root);

            assert_no_direct_nested_kind(&new_root, SyntaxKind::from_list);
        }
    }

    #[cfg(test)]
    mod learning_tests {
        use crate::cst;

        #[test]
        fn simple_format() {
            let input = "select a;";
            let root = cst::parse(input).unwrap();

            let actual = format!("{root}");
            let expected = "select a;";
            assert_eq!(actual, expected);
        }

        #[test]
        fn debug_formmat() {
            let input = "select a;";
            let root = cst::parse(input).unwrap();

            let actual = format!("{root:?}");
            let expected = "Root@0..9";
            assert_eq!(actual, expected);
        }

        #[test]
        fn pretty_debug_formmat() {
            let input = "select a;";
            let root = cst::parse(input).unwrap();

            let actual = format!("{root:#?}");
            let expected = r#"Root@0..9
  parse_toplevel@0..9
    stmtmulti@0..9
      stmtmulti@0..8
        toplevel_stmt@0..8
          stmt@0..8
            SelectStmt@0..8
              select_no_parens@0..8
                simple_select@0..8
                  SELECT@0..6 "select"
                  Whitespace@6..7 " "
                  opt_target_list@7..8
                    target_list@7..8
                      target_el@7..8
                        a_expr@7..8
                          c_expr@7..8
                            columnref@7..8
                              ColId@7..8
                                IDENT@7..8 "a"
      Semicolon@8..9 ";"
"#;
            assert_eq!(actual, expected);
        }
    }
}