use attributes::IdentList;
use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::iter::FromIterator;
use syn::{parse_macro_input, Attribute, AttributeArgs, Ident, ItemStruct};

mod attributes;
mod mapping;

#[derive(Debug, FromMeta)]
struct MappingOpts {
    #[darling(default)]
    exclude: Option<IdentList>,
    #[darling(default)]
    mutable: bool,
    #[darling(default)]
    groups: Option<IdentList>,
}

#[derive(Debug, Default, FromMeta)]
struct FieldOpts {
    /// Exclude this field from *all* mapping macros.
    #[darling(default)]
    exclude: bool,
    /// Exclude this field from the named groups.
    ///
    /// The group names should match groups defined on the `MappingOpts` for one or more mappings.
    // FIXME(sproul): we currently don't verify this
    #[darling(default)]
    exclude_from: Option<IdentList>,
}

/// Top-level configuration via the `metastruct` attribute.
#[derive(Debug, FromMeta)]
struct StructOpts {
    mappings: HashMap<Ident, MappingOpts>,
}

#[proc_macro_attribute]
pub fn metastruct(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let mut item = parse_macro_input!(input as ItemStruct);

    let type_name = &item.ident;

    let opts = StructOpts::from_list(&attr_args).unwrap();

    let mut output_items: Vec<TokenStream> = vec![];

    // Collect field names and types.
    let fields = item
        .fields
        .iter()
        .map(|field| (field.ident.clone().expect(""), field.ty.clone()))
        .collect::<Vec<_>>();

    // Collect field options.
    let field_opts = item
        .fields
        .iter()
        .map(|field| {
            field
                .attrs
                .iter()
                .filter(|attr| is_metastruct_attr(attr))
                .find_map(|attr| {
                    let meta = attr.parse_meta().unwrap();
                    Some(FieldOpts::from_meta(&meta).unwrap())
                })
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();

    // Generate mapping macros.
    for (mapping_macro_name, mapping_opts) in &opts.mappings {
        output_items.push(mapping::generate_mapping_macro(
            mapping_macro_name,
            type_name,
            &fields,
            &field_opts,
            mapping_opts,
        ));
    }

    // Output original struct definition after removing metastruct attributes from the fields.
    for field in &mut item.fields {
        field.attrs = discard_metastruct_attrs(&field.attrs);
    }
    output_items.push(quote! { #item }.into());

    TokenStream::from_iter(output_items)
}

/// Keep all non-metastruct-related attributes from an array.
fn discard_metastruct_attrs(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|attr| !is_metastruct_attr(attr))
        .cloned()
        .collect()
}

/// Predicate for determining whether an attribute is a `metastruct` attribute.
fn is_metastruct_attr(attr: &Attribute) -> bool {
    is_attr_with_ident(attr, "metastruct")
}

/// Predicate for determining whether an attribute has the given `ident` as its path.
fn is_attr_with_ident(attr: &Attribute, ident: &str) -> bool {
    attr.path
        .get_ident()
        .map_or(false, |attr_ident| attr_ident.to_string() == ident)
}
