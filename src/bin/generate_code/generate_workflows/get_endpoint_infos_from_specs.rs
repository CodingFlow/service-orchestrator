use oas3::spec::Operation;

use http::Method;

use oas3::spec::PathItem;

use crate::parse_specs::SpecInfo;

pub fn get_endpoint_infos_from_specs(
    spec_infos: Vec<SpecInfo>,
) -> Vec<(
    &'static SpecInfo,
    String,
    PathItem,
    Method,
    &'static Operation,
)> {
    spec_infos
        .iter()
        .flat_map(|spec_info| -> Vec<(&SpecInfo, String, &PathItem)> {
            spec_info
                .spec
                .paths
                .iter()
                .map(
                    |(path_string, path_item)| -> (&SpecInfo, String, &PathItem) {
                        (spec_info, path_string.to_string(), path_item)
                    },
                )
                .collect()
        })
        .flat_map(
            |(spec_info, path_string, path_item)| -> Vec<(&SpecInfo, String, PathItem, Method, &Operation)> {
                path_item
                    .methods()
                    .into_iter()
                    .map(
                        |(method, operation)| -> (&SpecInfo, String, PathItem, Method, &Operation) {
                            (spec_info, path_string.to_string(), path_item.clone(), method, operation)
                        },
                    )
                    .collect()
            },
        )
        .collect()
}
