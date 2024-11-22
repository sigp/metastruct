use crate::{exclude::calculate_excluded_fields, BiMappingOpts, FieldOpts, MappingOpts};
use itertools::Itertools;
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

    let mapping_function_input_types = selected_field_types
        .iter()
        .map(|field_type| {
            if mapping_opts.mutable {
                quote! { mut #field_type }
            } else {
                quote! { #field_type }
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
            ($lifetime:lifetime, $v:expr, $f:expr) => {
                match $v {
                    #type_name {
                        #(
                            #field_reference #selected_fields,
                        )*
                        ..
                    } => {
                        let mut __metastruct_i: usize = 0;
                        #(
                            let __metastruct_f: &mut dyn FnMut(usize, &$lifetime #mapping_function_input_types) -> _ = &mut $f;
                            #function_call_exprs;
                            __metastruct_i += 1;
                        )*
                    }
                }
            };
            ($v:expr, $f:expr) => {
                #macro_name!('_, $v, $f)
            };
        }
    }
    .into()
}

pub(crate) fn generate_bimapping_macro(
    macro_name: &Ident,
    left_type_name: &Ident,
    left_fields: &[(Ident, Type)],
    left_field_opts: &[FieldOpts],
    mapping_opts: &BiMappingOpts,
) -> TokenStream {
    let right_type_name = &mapping_opts.other_type;
    let exclude_idents = calculate_excluded_fields(
        &mapping_opts.exclude,
        &mapping_opts.groups,
        left_fields,
        left_field_opts,
    );
    let (left_selected_fields, right_selected_fields, left_selected_field_types): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = left_fields
        .iter()
        .filter(|(field_name, _)| !exclude_idents.contains(&field_name))
        .map(|(field_name, left_type)| {
            let right_field_name = Ident::new(&format!("{field_name}_r"), field_name.span());
            (field_name, right_field_name, left_type)
        })
        .multiunzip();

    assert!(
        !mapping_opts.self_by_value || !mapping_opts.self_mutable,
        "self cannot be mapped both by value and by mutable reference"
    );
    assert!(
        !mapping_opts.other_by_value || !mapping_opts.other_mutable,
        "other cannot be mapped both by value and by mutable reference"
    );
    let (left_field_ref, left_field_ref_typ) = if mapping_opts.self_by_value {
        (quote! {}, quote! {})
    } else if mapping_opts.self_mutable {
        (quote! { ref mut }, quote! { &'_ mut })
    } else {
        (quote! { ref }, quote! { &'_ })
    };
    let (right_field_ref, right_field_ref_typ) = if mapping_opts.other_by_value {
        (quote! {}, quote! {})
    } else if mapping_opts.other_mutable {
        (quote! { ref mut }, quote! { &'_ mut })
    } else {
        (quote! { ref }, quote! { &'_ })
    };

    let mapping_function_types = left_selected_field_types
        .iter()
        .map(|field_type| {
            quote! { &mut dyn FnMut(usize, #left_field_ref_typ #field_type, #right_field_ref_typ _) -> _ }
        })
        .collect::<Vec<_>>();

    let function_call_exprs = left_selected_fields
        .iter()
        .zip(&right_selected_fields)
        .map(|(left_field, right_field)| {
            if mapping_opts.fallible {
                quote! { __metastruct_f(__metastruct_i, #left_field, #right_field)? }
            } else {
                quote! { __metastruct_f(__metastruct_i, #left_field, #right_field) }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ($left:expr, $right:expr, $f:expr) => {
                match ($left, $right) {
                    (#left_type_name {
                        #(
                            #left_field_ref #left_selected_fields,
                        )*
                        ..
                    },
                    #right_type_name {
                        #(
                            #left_selected_fields: #right_field_ref #right_selected_fields,
                        )*
                        ..
                    }) => {
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
