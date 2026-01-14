use attributes::IdentList;
use darling::{export::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::iter::FromIterator;
use syn::{parse_macro_input, Attribute, Ident, ItemStruct};

mod attributes;
mod exclude;
mod mapping;
mod num_fields;

#[derive(Debug, FromMeta)]
struct MappingOpts {
    #[darling(default)]
    exclude: Option<IdentList>,
    #[darling(default)]
    mutable: bool,
    #[darling(default)]
    fallible: bool,
    #[darling(default)]
    groups: Option<IdentList>,
}

#[derive(Debug, FromMeta)]
struct BiMappingOpts {
    other_type: Ident,
    #[darling(default)]
    self_by_value: bool,
    #[darling(default)]
    self_mutable: bool,
    #[darling(default)]
    other_by_value: bool,
    #[darling(default)]
    other_mutable: bool,
    #[darling(default)]
    exclude: Option<IdentList>,
    #[darling(default)]
    fallible: bool,
    #[darling(default)]
    groups: Option<IdentList>,
}

#[derive(Debug, FromMeta)]
struct NumFieldsOpts {
    #[darling(default)]
    exclude: Option<IdentList>,
    #[darling(default)]
    selector: Option<Ident>,
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
    #[darling(default)]
    mappings: HashMap<Ident, MappingOpts>,
    #[darling(default)]
    bimappings: HashMap<Ident, BiMappingOpts>,
    // FIXME(sproul): the `Ident` is kind of useless here, consider writing a custom FromMeta
    #[darling(default)]
    num_fields: HashMap<Ident, NumFieldsOpts>,
}

#[proc_macro_attribute]
pub fn metastruct(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(args) => args,
        Err(err) => return err.to_compile_error().into(),
    };
    let mut item = parse_macro_input!(input as ItemStruct);

    let type_name = &item.ident;

    // Generics used for impl blocks.
    let generics = &item.generics.split_for_impl();

    let opts = match StructOpts::from_list(&attr_args) {
        Ok(opts) => opts,
        Err(err) => return err.write_errors().into(),
    };

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
                    let meta = &attr.meta;
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

    // Generate bi-mapping macros.
    for (mapping_macro_name, mapping_opts) in &opts.bimappings {
        output_items.push(mapping::generate_bimapping_macro(
            mapping_macro_name,
            type_name,
            &fields,
            &field_opts,
            mapping_opts,
        ));
    }

    // Generate `NumFields` implementations.
    for (_, num_fields_opts) in &opts.num_fields {
        output_items.push(num_fields::generate_num_fields_impl(
            type_name,
            generics,
            &fields,
            &field_opts,
            num_fields_opts,
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
    attr.path()
        .get_ident()
        .map_or(false, |attr_ident| attr_ident.to_string() == ident)
}
