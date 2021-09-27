use crate::syntax::{SyntaxElement, SyntaxKind, SyntaxNode};
use rowan::TextSize;

mod escape;
pub mod syntax;
pub mod coords;

pub use escape::check_escape;
pub use escape::unescape;

pub(crate) mod allowed_chars {
    pub(crate) fn comment(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t' && c.is_control() {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() {
            Ok(())
        } else {
            Err(err_indices)
        }
    }

    pub(crate) fn string(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t'
                && (c >= '\u{0000}' && c <= '\u{0008}'
                    || c >= '\u{000A}' && c <= '\u{001F}'
                    || c == '\u{007F}')
            {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() {
            Ok(())
        } else {
            Err(err_indices)
        }
    }

    pub(crate) fn multi_line_string(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t'
                && c != '\n'
                && c != '\r'
                && (c >= '\u{0000}' && c <= '\u{0008}'
                    || c >= '\u{000A}' && c <= '\u{001F}'
                    || c == '\u{007F}')
            {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() {
            Ok(())
        } else {
            Err(err_indices)
        }
    }

    pub(crate) fn string_literal(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t' && c.is_control() {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() {
            Ok(())
        } else {
            Err(err_indices)
        }
    }

    pub(crate) fn multi_line_string_literal(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t' && c != '\n' && c != '\r' && c.is_control() {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() {
            Ok(())
        } else {
            Err(err_indices)
        }
    }
}

pub trait StrExt {
    fn strip_quotes(self) -> Self;
}

impl StrExt for &str {
    fn strip_quotes(self) -> Self {
        if self.starts_with('\"') || self.starts_with('\'') {
            &self[1..self.len() - 1]
        } else {
            self
        }
    }
}

/// Utility extension methods for Syntax Nodes.
pub trait SyntaxExt {
    /// Return a syntax node that contains the given offset.
    fn find_node(&self, offset: TextSize, inclusive: bool) -> Option<SyntaxNode>;

    /// Find the deepest node that contains the given offset.
    fn find_node_deep(&self, offset: TextSize, inclusive: bool) -> Option<SyntaxNode> {
        let mut node = self.find_node(offset, inclusive);
        while let Some(n) = &node {
            let new_node = n.find_node(offset, inclusive);
            if new_node.is_some() {
                node = new_node;
            } else {
                break;
            }
        }

        node
    }

    /// Find a node or token by its kind.
    fn find(&self, kind: SyntaxKind) -> Option<SyntaxElement>;
}

impl SyntaxExt for SyntaxNode {
    fn find_node(&self, offset: TextSize, inclusive: bool) -> Option<SyntaxNode> {
        for d in self.descendants().skip(1) {
            let range = d.text_range();

            if (inclusive && range.contains_inclusive(offset)) || range.contains(offset) {
                return Some(d);
            }
        }

        None
    }

    fn find(&self, kind: SyntaxKind) -> Option<SyntaxElement> {
        for d in self.descendants_with_tokens() {
            if d.kind() == kind {
                return Some(d);
            }
        }

        None
    }
}
