use crate::{Document, utils::LspExt};
use lsp_types::{DocumentSymbol, SymbolKind};
use rowan::TextRange;
use taplo::{
    dom::{NodeSyntax, ValueNode},
    util::{coords::Mapper, syntax::join_ranges},
};

pub(crate) fn create_symbols(doc: &Document) -> Vec<DocumentSymbol> {
    let mapper = &doc.mapper;
    let mut symbols: Vec<DocumentSymbol> = Vec::new();

    for (_, entry) in doc.parse.clone().into_dom().entries().iter() {
        symbols_for_value(
            ensure_non_empty_key(entry.key().full_key_string()),
            None,
            entry.value(),
            mapper,
            &mut symbols,
        );
    }

    symbols
}

#[allow(deprecated)]
fn symbols_for_value(
    name: String,
    key_range: Option<TextRange>,
    value: &ValueNode,
    mapper: &Mapper,
    symbols: &mut Vec<DocumentSymbol>,
) {
    let own_range = mapper.range(join_ranges(value.text_ranges())).unwrap();

    let range = if let Some(key_r) = key_range {
        mapper
            .range(key_r.cover(join_ranges(value.text_ranges())))
            .unwrap()
    } else {
        own_range
    };

    let selection_range = key_range
        .map(|r| mapper.range(r).unwrap())
        .unwrap_or(own_range);

    match value {
        ValueNode::Bool(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::Boolean,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        ValueNode::String(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::String,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        ValueNode::Integer(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::Number,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        ValueNode::Float(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::Number,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        ValueNode::Date(_) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::Field,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: None,
        }),
        ValueNode::Array(arr) => symbols.push(DocumentSymbol {
            name,
            kind: SymbolKind::Array,
            range: range.into_lsp(),
            selection_range: selection_range.into_lsp(),
            detail: None,
            deprecated: None,
            tags: Default::default(),
            children: {
                let mut child_symbols = Vec::with_capacity(arr.items().len());

                for (i, c) in arr.items().iter().enumerate() {
                    symbols_for_value(i.to_string(), None, c, mapper, &mut child_symbols);
                }

                Some(child_symbols)
            },
        }),
        ValueNode::Table(t) => {
            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::Object,
                range: range.into_lsp(),
                selection_range: selection_range.into_lsp(),
                detail: None,
                deprecated: None,
                tags: Default::default(),
                children: {
                    let mut child_symbols = Vec::with_capacity(t.entries().len());

                    for (_, c) in t.entries().iter() {
                        symbols_for_value(
                            c.key().full_key_string(),
                            Some(c.key().syntax().text_range()),
                            c.value(),
                            mapper,
                            &mut child_symbols,
                        );
                    }

                    Some(child_symbols)
                },
            });
        }
        _ => {}
    }
}

fn ensure_non_empty_key(s: String) -> String {
    if s.is_empty() {
        r#"''"#.into()
    } else {
        s
    }
}
