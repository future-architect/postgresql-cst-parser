use crate::{syntax_kind::SyntaxKind, ResolvedNode};

/// Asserts that there is at least one node of the specified `SyntaxKind` in the given syntax tree.
pub fn assert_exists(root: &ResolvedNode, kind: SyntaxKind) {
    let exists = root.descendants().any(|node| node.kind() == kind);
    assert!(
        exists,
        "Expected at least one node of kind {kind:?}, but none was found."
    )
}

/// Asserts that there are no nodes of the specified `SyntaxKind` in the given syntax tree.
pub fn assert_not_exists(root: &ResolvedNode, kind: SyntaxKind) {
    let exists = root.descendants().any(|node| node.kind() == kind);
    assert!(
        !exists,
        "Expected no nodes of kind {kind:?}, but at least one was found."
    )
}

/// Asserts that the exact number of nodes of the specified `SyntaxKind` matches the given count.
pub fn assert_node_count(root: &ResolvedNode, kind: SyntaxKind, expected_count: usize) {
    let actual_count = root
        .descendants()
        .filter(|node| node.kind() == kind)
        .count();
    assert_eq!(
        actual_count, expected_count,
        "Expected {expected_count} nodes of kind {kind:?}, but found {actual_count}."
    )
}

/// Asserts that there are no directly nested nodes of the specified `SyntaxKind`.
/// In other words, a node of `kind` cannot have another `kind` node as its immediate child.
pub fn assert_no_direct_nested_kind(root: &ResolvedNode, kind: SyntaxKind) {
    let target_nodes = root.descendants().filter(|node| node.kind() == kind);

    for node in target_nodes {
        if let Some(parent) = node.parent() {
            assert!(
                !(node.kind() == kind && parent.kind() == kind),
                "Found a `{parent:?}` node that directly contains another {kind:?} node as a child."
            )
        }
    }
}

/// Asserts that there is at least one directly nested node of the specified `SyntaxKind`.
/// In other words, there must be a node of `kind` that has another `kind` node as its immediate child.
pub fn assert_direct_nested_kind(root: &ResolvedNode, kind: SyntaxKind) {
    let has_direct_nesting = root
        .descendants()
        .filter(|node| node.kind() == kind)
        .any(|node| {
            if let Some(parent) = node.parent() {
                node.kind() == kind && parent.kind() == kind
            } else {
                false
            }
        });

    assert!(
        has_direct_nesting,
        "Expected at least one directly nested {kind:?} node, but none was found."
    );
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
    #[should_panic(expected = "Expected at least one node of kind InsertStmt, but none was found.")]
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

    #[test]
    fn test_direct_nested_kind_passes() {
        let input = "select a,b,c;";
        let root = cst::parse(input).unwrap();

        assert_direct_nested_kind(&root, SyntaxKind::target_list);
    }

    #[test]
    #[should_panic(
        expected = "Expected at least one directly nested SelectStmt node, but none was found."
    )]
    fn test_direct_nested_kind_fails() {
        let input = "select a;";
        let root = cst::parse(input).unwrap();

        assert_direct_nested_kind(&root, SyntaxKind::SelectStmt);
    }
}
