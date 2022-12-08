use crate::{attributes::IdentList, FieldOpts};
use syn::{Ident, Type};

pub(crate) fn calculate_excluded_fields<'a>(
    item_excludes: &'a Option<IdentList>,
    item_groups: &Option<IdentList>,
    fields: &'a [(Ident, Type)],
    field_opts: &[FieldOpts],
) -> Vec<&'a Ident> {
    item_excludes
        .as_ref()
        .into_iter()
        .map(|exclude| &exclude.idents)
        .flatten()
        .chain(
            fields
                .iter()
                .zip(field_opts)
                .filter_map(|((field_name, _), field_opts)| {
                    let excluded_from_all = field_opts.exclude;
                    let excluded_from_any_group = item_groups
                        .as_ref()
                        .and_then(|groups| {
                            let excluded_groups = field_opts.exclude_from.as_ref()?;
                            Some(
                                groups
                                    .idents
                                    .iter()
                                    .any(|group| excluded_groups.idents.contains(group)),
                            )
                        })
                        .unwrap_or(false);
                    (excluded_from_all || excluded_from_any_group).then_some(field_name)
                }),
        )
        .collect()
}
