use crate::MappingOpts;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub(crate) fn generate_mapping_macro(
    macro_name: &Ident,
    type_name: &Ident,
    fields: &[(Ident, Type)],
    mapping_opts: &MappingOpts,
) -> TokenStream {
    let exclude_idents = mapping_opts
        .exclude
        .as_ref()
        .map(|x| x.idents.as_slice())
        .unwrap_or(&[]);
    let (selected_fields, selected_field_types): (Vec<_>, Vec<_>) = fields
        .iter()
        .filter(|(field_name, _)| !exclude_idents.contains(&field_name))
        .cloned()
        .unzip();

    let field_reference = if mapping_opts.mutable {
        quote! { ref mut }
    } else {
        quote! { ref }
    };

    let mapping_function_types = selected_field_types
        .iter()
        .map(|field_type| {
            if mapping_opts.mutable {
                quote! { &mut dyn FnMut(usize, &'_ mut #field_type) -> _ }
            } else {
                quote! { &mut dyn FnMut(usize, &'_ #field_type) -> _ }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ($v:expr, $f:expr) => {
                match $v {
                    #type_name {
                        #(
                            #field_reference #selected_fields,
                        )*
                        ..
                    } => {
                        let mut __metastruct_i: usize = 0;
                        #(
                            let __metastruct_f: #mapping_function_types = &mut $f;
                            __metastruct_f(__metastruct_i, #selected_fields);
                            __metastruct_i += 1;
                        )*
                    }
                }
            }
        }
    }
    .into()
}

/*
// FIXME(sproul): mutability
fn generate_mapping_macro(
    macro_name: &Ident,
    type_name: &Ident,
    fields: &[(Ident, Type)],
    mapping_opts: &MappingOpts,
) -> TokenStream {
    let exclude_idents = &mapping_opts.exclude.idents;
    let (selected_fields, selected_field_types): (Vec<_>, Vec<_>) = fields
        .iter()
        .filter(|(field_name, _)| !exclude_idents.contains(&field_name))
        .cloned()
        .unzip();

    quote! {
        macro_rules! #name {
            ($v:expr, $f:expr) => {
                match $v {
                    #type_name {
                        #(
                            ref #selected_fields
                        ),*
                    } => {
                        #(
                            let __metastruct_f: &mut dyn FnMut(&'_ #selected_field_types) -> _ = &mut $f;
                            __metastruct_f(#selected_fields);
                        )*
                    }
                }
            }
        }
    }
}
*/
