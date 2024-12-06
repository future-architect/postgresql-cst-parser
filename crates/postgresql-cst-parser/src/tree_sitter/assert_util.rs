use crate::{syntax_kind::SyntaxKind, ResolvedNode};

pub fn assert_exists(root: &ResolvedNode, kind: SyntaxKind) {
    let exists = root.descendants().any(|node| node.kind() == kind);
    if !exists {
        panic!(
            "Expected no nodes of kind {:?}, but at least one was found.",
            kind
        );
    }
}

pub fn assert_not_exists(root: &ResolvedNode, kind: SyntaxKind) {
    let exists = root.descendants().any(|node| node.kind() == kind);
    if exists {
        panic!(
            "Expected no nodes of kind {:?}, but at least one was found.",
            kind
        );
    }
}

pub fn assert_node_count(root: &ResolvedNode, kind: SyntaxKind, expected_count: usize) {
    let actual_count = root
        .descendants()
        .filter(|node| node.kind() == kind)
        .count();
    assert_eq!(
        actual_count, expected_count,
        "Expected {} nodes of kind {:?}, but found {}.",
        expected_count, kind, actual_count
    )
}

pub fn assert_no_direct_nested_kind(root: &ResolvedNode, kind: SyntaxKind) {
    fn visit(node: &ResolvedNode, kind: SyntaxKind) {
        if node.kind() == kind {
            for child in node.children() {
                if child.kind() == kind {
                    panic!(
                        "Found a `{:?}` node that directly contains another {:?} node as a child.",
                        node, kind
                    );
                }
            }
        }

        for child in node.children() {
            visit(&child, kind);
        }
    }
    visit(root, kind);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cst;

    #[test]
    fn test_assert_exists_passes() {
        let input = "select a, b, c from t;";
        let root = cst::parse(input).unwrap();
        assert_exists(&root, SyntaxKind::SelectStmt);
        assert_exists(&root, SyntaxKind::from_clause);
    }

    #[test]
    #[should_panic(expected = "Expected no nodes of kind InsertStmt, but at least one was found.")]
    fn test_assert_exists_fails() {
        let input = "select a, b, c from t;";
        let root = cst::parse(input).unwrap();
        assert_exists(&root, SyntaxKind::InsertStmt);
    }

    #[test]
    fn test_assert_not_exists_passes() {
        let input = "select a, b, c from t;";
        let root = cst::parse(input).unwrap();
        assert_not_exists(&root, SyntaxKind::InsertStmt);
        assert_not_exists(&root, SyntaxKind::with_clause);
    }

    #[test]
    #[should_panic(expected = "Expected no nodes of kind from_clause, but at least one was found.")]
    fn test_assert_not_exists_fails() {
        let input = "select a, b, c from t;";
        let root = cst::parse(input).unwrap();
        assert_not_exists(&root, SyntaxKind::from_clause);
    }
    #[test]
    fn test_assert_node_count_passes() {
        let input = "select a, b, c from t;";
        let root = cst::parse(input).unwrap();

        assert_node_count(&root, SyntaxKind::SelectStmt, 1);
        assert_node_count(&root, SyntaxKind::target_el, 3);
        assert_node_count(&root, SyntaxKind::from_clause, 1);
        assert_node_count(&root, SyntaxKind::DeleteStmt, 0);
    }
    #[test]
    #[should_panic(expected = "Expected 0 nodes of kind SelectStmt, but found 1.")]
    fn test_assert_node_count_fails() {
        let input = "select a, b, c from t;";
        let root = cst::parse(input).unwrap();

        assert_node_count(&root, SyntaxKind::SelectStmt, 0);
    }

    #[test]
    fn test_no_direct_nested_kind_passes() {
        let input = "select a;";
        let root = cst::parse(input).unwrap();

        assert_no_direct_nested_kind(&root, SyntaxKind::target_list);
    }

    #[test]
    #[should_panic(
        expected = "Found a `target_list@7..12` node that directly contains another target_list node as a child."
    )]
    fn test_no_direct_nested_kind_fails() {
        let input = "select a,b,c;";
        let root = cst::parse(input).unwrap();

        assert_no_direct_nested_kind(&root, SyntaxKind::target_list);
    }
}
