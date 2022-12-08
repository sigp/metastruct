use crate::{exclude::calculate_excluded_fields, FieldOpts, MappingOpts};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub(crate) fn generate_mapping_macro(
    macro_name: &Ident,
    type_name: &Ident,
    fields: &[(Ident, Type)],
    field_opts: &[FieldOpts],
    mapping_opts: &MappingOpts,
) -> TokenStream {
    let exclude_idents = calculate_excluded_fields(
        &mapping_opts.exclude,
        &mapping_opts.groups,
        fields,
        field_opts,
    );
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

    let function_call_exprs = selected_fields
        .iter()
        .map(|field| {
            if mapping_opts.fallible {
                quote! { __metastruct_f(__metastruct_i, #field)? }
            } else {
                quote! { __metastruct_f(__metastruct_i, #field) }
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
                            #function_call_exprs;
                            __metastruct_i += 1;
                        )*
                    }
                }
            }
        }
    }
    .into()
}
