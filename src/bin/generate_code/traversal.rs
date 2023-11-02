#[derive(Clone, Debug)]
pub struct NestedNode<T> {
    pub current: T,
    pub children: Option<Vec<NestedNode<T>>>,
}

/// Traverses a nested hierachy of types, processes them,
/// and creates a hiearchy of [NestedNode] with the same structure.
pub fn traverse_nested_type<T: Clone, R: Clone, U>(
    current: T,
    action: fn(T, &mut U) -> R,
    nested_action: fn(child: T, parent_result: &mut R, &mut U),
    nested_reference: fn(current: T) -> Option<Vec<T>>,
    additional_action_input: &mut U,
    recurse_only_if_child_has_children: bool,
) -> NestedNode<R> {
    let mut action_result = action(current.clone(), additional_action_input);
    let mut nested_results = None;

    if let Some(children) = nested_reference(current.clone()) {
        nested_results = {
            let mut result = vec![];

            for child in children {
                let child = child.clone();
                nested_action(child.clone(), &mut action_result, additional_action_input);

                if !recurse_only_if_child_has_children || nested_reference(child.clone()).is_some()
                {
                    let child_result = traverse_nested_type(
                        child.clone(),
                        action,
                        nested_action,
                        nested_reference,
                        additional_action_input,
                        recurse_only_if_child_has_children,
                    );

                    result.push(child_result);
                }
            }

            match result.len().gt(&0) {
                true => Some(result),
                false => None,
            }
        };
    }

    NestedNode {
        current: action_result,
        children: nested_results,
    }
}

/// Maps contents of [NestedNode] structure.
// pub fn map_nested_node<T: Clone, R: Clone>(
//     current: NestedNode<T>,
//     action: fn(NestedNode<T>) -> R,
//     nested_action: fn(child: NestedNode<T>, parent_result: R),
// ) -> NestedNode<R> {
//     traverse_nested_type(current.clone(), action, nested_action, |current| {
//         current.clone().children
//     })
// }

/// Traverses a nested hierachy of [NestedNode]s and processes them.
pub fn traverse_nested_node<T: Clone, R: Clone>(
    current: NestedNode<T>,
    action: impl Fn(NestedNode<T>) -> R,
    nested_action: fn(child: NestedNode<T>, parent_result: R),
) {
    let action_result = action(current.clone());

    if let Some(ref children) = current.children {
        for child in children {
            nested_action(child.clone(), action_result.clone());
            traverse_nested_node(child.clone(), &action, nested_action)
        }
    }
}
