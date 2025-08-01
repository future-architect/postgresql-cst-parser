use crate::{lexer::TokenKind, syntax_kind::SyntaxKind};

use super::{LRParseState, ParseTransform, ParseTransformer};

/// Skip extra commas at the beginning of SELECT, FROM, and ORDER BY clauses
pub struct SkipExtraComma;

impl SkipExtraComma {
    fn allow_extra_comma<'a>(lr_parse_state: &LRParseState<'a>) -> bool {
        match lr_parse_state.last_syntax_kind() {
            Some(SyntaxKind::FROM)
            | Some(SyntaxKind::BY)
            | Some(SyntaxKind::SELECT)
            | Some(SyntaxKind::ALL)
            | Some(SyntaxKind::DISTINCT) => true,

            Some(SyntaxKind::RParen) => {
                // select distinct on
                let expected_select_distinct_on_kinds = [
                    SyntaxKind::RParen,
                    SyntaxKind::expr_list,
                    SyntaxKind::LParen,
                    SyntaxKind::ON,
                    SyntaxKind::DISTINCT,
                    SyntaxKind::SELECT,
                ];

                let actual_kinds: Vec<_> = lr_parse_state
                    .iter_syntax_kind_rev()
                    .take(expected_select_distinct_on_kinds.len())
                    .collect();

                expected_select_distinct_on_kinds.as_ref() == actual_kinds.as_slice()
            }
            _ => false,
        }
    }
}

impl ParseTransformer for SkipExtraComma {
    fn transform<'a>(&self, lr_parse_state: &LRParseState<'a>) -> Option<ParseTransform> {
        if !Self::allow_extra_comma(lr_parse_state) {
            return None;
        }

        if lr_parse_state.token.kind != TokenKind::RAW(",".to_string()) {
            return None;
        }

        Some(ParseTransform::SkipToken)
    }
}
