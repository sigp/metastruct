use crate::{exclude::calculate_excluded_fields, FieldOpts, NumFieldsOpts};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ImplGenerics, Type, TypeGenerics, WhereClause};

pub(crate) fn generate_num_fields_impl(
    type_name: &Ident,
    (impl_generics, ty_generics, where_clause): &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    fields: &[(Ident, Type)],
    field_opts: &[FieldOpts],
    num_fields_opts: &NumFieldsOpts,
) -> TokenStream {
    let (selector_ty, selector_ty_def) = if let Some(selector) = &num_fields_opts.selector {
        (quote! { #selector }, Some(quote! { pub enum #selector {} }))
    } else {
        (quote! { metastruct::selectors::AllFields }, None)
    };

    let excluded_fields = calculate_excluded_fields(
        &num_fields_opts.exclude,
        &num_fields_opts.groups,
        fields,
        field_opts,
    );
    let num_fields = fields
        .iter()
        .filter(|(field_name, _)| !excluded_fields.contains(&field_name))
        .count();

    quote! {
        #selector_ty_def

        impl #impl_generics metastruct::NumFields<#selector_ty> for #type_name #ty_generics
        #where_clause
        {
            const NUM_FIELDS: usize = #num_fields;
        }
    }
    .into()
}
