#[derive(Clone, Debug)]
pub struct NestedNode<T> {
    pub current: T,
    pub children: Option<Vec<NestedNode<T>>>,
}

/// Traverses a nested hierachy of types, processes them,
/// and creates a hiearchy of [NestedNode] with the same structure.
pub fn traverse_nested_type<T: Clone, R: Clone>(
    current: T,
    action: fn(T) -> R,
    nested_action: fn(child: T, parent_result: R),
    nested_reference: fn(current: T) -> Option<Vec<T>>,
) -> NestedNode<R> {
    let action_result = action(current.clone());
    let mut nested_results = None;

    if let Some(children) = nested_reference(current.clone()) {
        nested_results = fun_name(
            children,
            nested_action,
            &action_result,
            action,
            nested_reference,
        );
    }

    NestedNode {
        current: action_result,
        children: nested_results,
    }
}

fn fun_name<T: Clone, R: Clone>(
    children: Vec<T>,
    nested_action: fn(T, R),
    action_result: &R,
    action: fn(T) -> R,
    nested_reference: fn(T) -> Option<Vec<T>>,
) -> Option<Vec<NestedNode<R>>> {
    let mut result = vec![];

    for child in children {
        result.push(inner_fn(
            child.clone(),
            action_result.clone(),
            action,
            nested_action,
            nested_reference,
        ));
    }

    Some(result)
}

fn inner_fn<T: Clone, R: Clone>(
    child: T,
    action_result: R,
    action: fn(T) -> R,
    nested_action: fn(child: T, parent_result: R),
    nested_reference: fn(current: T) -> Option<Vec<T>>,
) -> NestedNode<R> {
    nested_action(child.clone(), action_result.clone());
    traverse_nested_type(child.clone(), action, nested_action, nested_reference)
}

/// Maps contents of [NestedNode] structure.
pub fn map_nested_node<T: Clone, R: Clone>(
    current: NestedNode<T>,
    action: fn(NestedNode<T>) -> R,
    nested_action: fn(child: NestedNode<T>, parent_result: R),
) -> NestedNode<R> {
    traverse_nested_type(current.clone(), action, nested_action, |current| {
        current.clone().children
    })
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
