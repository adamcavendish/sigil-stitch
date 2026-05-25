use proc_macro2::TokenTree;

pub(super) fn is_semicolon(tt: &TokenTree) -> bool {
    matches!(tt, TokenTree::Punct(p) if p.as_char() == ';')
}

pub(super) fn is_ident(tt: &TokenTree, name: &str) -> bool {
    matches!(tt, TokenTree::Ident(id) if *id == name)
}
