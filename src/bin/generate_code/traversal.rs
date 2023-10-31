pub fn nested_traverse_general<T: Clone, R: Clone>(
    current: T,
    action: impl Fn(T) -> R,
    nested_action: fn(child: T, parent_result: R),
    nested_reference: fn(current: T) -> Option<Vec<T>>,
) {
    let action_result = action(current.clone());

    if let Some(ref children) = nested_reference(current.clone()) {
        for child in children {
            nested_action(child.clone(), action_result.clone());
            nested_traverse_general(child.clone(), &action, nested_action, nested_reference);
        }
    }
}
