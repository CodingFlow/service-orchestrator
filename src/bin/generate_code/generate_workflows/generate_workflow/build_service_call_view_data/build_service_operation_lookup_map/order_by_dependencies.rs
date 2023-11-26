use super::ServiceCodeGenerationInfo;
use std::collections::BTreeMap;

/// Returns [ServiceCodeGenerationInfo]s ordered from most dependent to independent
/// e.g. a -> b -> c then \[a, b, c\]
pub fn order_by_dependencies(
    code_generation_infos: BTreeMap<(String, String), ServiceCodeGenerationInfo>,
) -> Vec<((String, String), ServiceCodeGenerationInfo)> {
    let mut vector: Vec<((String, String), ServiceCodeGenerationInfo)> =
        code_generation_infos.into_iter().collect();

    let mut complete = false;

    while !complete {
        let vec = &vector.to_vec();
        let iter = vec.iter();
        let cloned_vector: Vec<(usize, &((String, String), ServiceCodeGenerationInfo))> =
            iter.clone().enumerate().collect();

        complete = reorder_loop(cloned_vector, iter, &mut vector);
    }

    vector
}

fn reorder_loop(
    cloned_vector: Vec<(usize, &((String, String), ServiceCodeGenerationInfo))>,
    iter: std::slice::Iter<'_, ((String, String), ServiceCodeGenerationInfo)>,
    vector: &mut Vec<((String, String), ServiceCodeGenerationInfo)>,
) -> bool {
    for (index, (_, info)) in cloned_vector {
        let depending_services = &info.dependencies_service_operation_names;
        let misplaced_depending_service_index =
            iter.clone().take(index).position(|(service_operation, _)| {
                depending_services
                    .iter()
                    .any(|service| service == service_operation)
            });

        if let Some(misplaced_index) = misplaced_depending_service_index {
            let misplaced_service = vector.remove(misplaced_index);
            vector.insert(index, misplaced_service);
            return false;
        }
    }

    true
}
