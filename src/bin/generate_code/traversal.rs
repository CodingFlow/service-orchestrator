#[derive(Clone, Debug)]
pub struct NestedNode<T> {
    pub current: T,
    pub children: Option<Vec<NestedNode<T>>>,
}

/// Traverses a nested hierachy of types, processes them,
/// and creates a hiearchy of [NestedNode] with the same structure.
pub fn traverse_nested_type<T: Clone, R: Clone>(
    current: T,
    action: impl Fn(T) -> R,
    nested_action: fn(child: T, parent_result: R),
    nested_reference: fn(current: T) -> Option<Vec<T>>,
) -> NestedNode<R> {
    let action_result = action(current.clone());
    let mut nested_results = None;

    if let Some(ref children) = nested_reference(current.clone()) {
        nested_results = Some(
            children
                .iter()
                .map(|child| -> NestedNode<R> {
                    nested_action(child.clone(), action_result.clone());
                    traverse_nested_type(child.clone(), &action, nested_action, nested_reference)
                })
                .collect(),
        );
    }

    NestedNode {
        current: action_result,
        children: nested_results,
    }
}

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
