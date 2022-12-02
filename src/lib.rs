use attributes::IdentList;
use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::iter::FromIterator;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemStruct};

mod attributes;
mod mapping;

#[derive(Debug, FromMeta)]
struct MappingOpts {
    #[darling(default)]
    exclude: Option<IdentList>,
    #[darling(default)]
    mutable: bool,
}

/// Top-level configuration via the `metastruct` attribute.
#[derive(Debug, FromMeta)]
struct StructOpts {
    mappings: HashMap<Ident, MappingOpts>,
}

#[proc_macro_attribute]
pub fn metastruct(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let item = parse_macro_input!(input as ItemStruct);

    let type_name = &item.ident;

    let opts = StructOpts::from_list(&attr_args).unwrap();

    let mut output_items: Vec<TokenStream> = vec![];

    // Collect field names and types.
    let fields = item
        .fields
        .iter()
        .map(|field| (field.ident.clone().expect(""), field.ty.clone()))
        .collect::<Vec<_>>();

    // Generate mapping macros.
    for (mapping_macro_name, mapping_opts) in &opts.mappings {
        output_items.push(mapping::generate_mapping_macro(
            mapping_macro_name,
            type_name,
            &fields,
            mapping_opts,
        ));
    }

    // Output original struct definition unchanged.
    output_items.push(quote! { #item }.into());

    TokenStream::from_iter(output_items)
}
