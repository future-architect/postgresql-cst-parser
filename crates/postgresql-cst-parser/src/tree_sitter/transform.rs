use cstree::{build::GreenNodeBuilder, syntax::SyntaxNode};

use crate::{syntax_kind::SyntaxKind, PostgreSQLSyntax, ResolvedNode};

/// Function call flow:
///  transform_cst (public)
///    +- walk_and_build
///      +- flatten
///      +- remove_exact

pub fn transform_cst(root: &ResolvedNode) -> ResolvedNode {
    let mut builder = GreenNodeBuilder::new();

    builder.start_node(SyntaxKind::Root);
    walk_and_build(&mut builder, root);
    builder.finish_node();

    let (tree, cache) = builder.finish();
    let new_root =
        SyntaxNode::new_root_with_resolver(tree, cache.unwrap().into_interner().unwrap());

    new_root
}

/// CST を走査し、いくつかの Node を書き換える
/// e.g. flatten list node, remove option node
fn walk_and_build(
    builder: &mut GreenNodeBuilder<'static, 'static, PostgreSQLSyntax>,
    node: &ResolvedNode,
) {
    use cstree::util::NodeOrToken;
    let parent_kind = node.kind();
    let mut children = node.children_with_tokens();

    while let Some(child) = children.next() {
        match child {
            NodeOrToken::Node(n) => {
                // for debug
                if format!("{}", n.kind()).starts_with("opt_") {
                    println!("Node  (kind: {})", n.kind());
                }

                match n.kind() {
                    child_kind @ SyntaxKind::target_list => {
                        if parent_kind == child_kind {
                            // [Flatten]
                            //
                            // This patten does not construct node.
                            //
                            // * target_list (parent)   <- 1. A node that passed as an argument of this function.
                            //   +- target_el           <- 2. This token was already consumed in previous loop.
                            //   +- target_list (child) <- 3. This is the nested node (parent is the same syntax kind).  Just ignore this node, and continue building its children.
                            //     +- target_el 
                            //     +- ...
                            // 
                            walk_and_build(builder, n);
                        } else {
                            builder.start_node(n.kind());
                            walk_and_build(builder, n);
                            builder.finish_node();
                        }
                    }

                    SyntaxKind::opt_target_list => {
                        // [Removal]
                        //
                        // just ignore current node, and continue building its children.
                        //
                        // (Old Tree)                                                   (New Tree) 
                        // *- parent_node                  (ignore opt_target_list)     *- parent_node
                        //    +- opt_target_list          =========================>       +- child_1
                        //       +- child_1                                                +- child_2
                        //       +- child_1
                        //
                        walk_and_build(builder, n);
                    }

                    // all pattern
                    kind @ _ => {
                        builder.start_node(kind);
                        walk_and_build(builder, n);
                        builder.finish_node();
                    }
                }
            }
            NodeOrToken::Token(t) => {
                // for debug
                // println!(
                //     "Token (kind: {}, text: \"{}\")",
                //     t.kind(),
                //     t.text().escape_debug()
                // );

                builder.token(t.kind(), t.text());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cst, tree_sitter::transform::transform_cst};

    #[test]
    fn restored_texts_are_equal() {
        let input = r#"
SELECT
	1 as X
,	2
,	3
FROM
	A
,	B"#;
        // dbg!(input);
        let root = cst::parse(input).unwrap();

        let new_root = transform_cst(&root);

        // dbg!(&root);
        // dbg!(&new_root);
        assert_eq!(format!("{root}"), format!("{new_root}"));
    }
    mod removal {
        use crate::{cst, tree_sitter::transform::transform_cst};

        #[test]
        fn opt_target_list() {
            let input = "select a,b,c;";
            let root = cst::parse(input).unwrap();
            let new_root = transform_cst(&root);

            let printed_new_tree = format!("{new_root:#?}");
            assert!(!printed_new_tree.contains("opt_target_list"))
        }
    }

    mod flatten {
        use crate::{cst, tree_sitter::transform::transform_cst};

        #[test]
        fn target_list() {
            let input = "select a,b,c;";
            let root = cst::parse(input).unwrap();
            let new_root = transform_cst(&root);

            let actual = format!("{new_root:#?}");
            let expected = r#"Root@0..13
  parse_toplevel@0..13
    stmtmulti@0..13
      stmtmulti@0..12
        toplevel_stmt@0..12
          stmt@0..12
            SelectStmt@0..12
              select_no_parens@0..12
                simple_select@0..12
                  SELECT@0..6 "select"
                  Whitespace@6..7 " "
                  target_list@7..12
                    target_el@7..8
                      a_expr@7..8
                        c_expr@7..8
                          columnref@7..8
                            ColId@7..8
                              IDENT@7..8 "a"
                    Comma@8..9 ","
                    target_el@9..10
                      a_expr@9..10
                        c_expr@9..10
                          columnref@9..10
                            ColId@9..10
                              IDENT@9..10 "b"
                    Comma@10..11 ","
                    target_el@11..12
                      a_expr@11..12
                        c_expr@11..12
                          columnref@11..12
                            ColId@11..12
                              IDENT@11..12 "c"
      Semicolon@12..13 ";"
"#;

            assert_eq!(actual, expected);
        }
        
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
