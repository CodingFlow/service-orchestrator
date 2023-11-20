#[derive(Clone, Debug)]
pub struct NestedNode<T> {
    pub current: T,
    pub children: Option<Vec<NestedNode<T>>>,
}

/// Converts a nested type into [NestedNode] by wrapping the nested types.
pub fn convert_to_nested_node<T, R, U>(
    current: T,
    action: fn(T, &mut U) -> R,
    nested_reference: fn(current: T, additional_action_input: &mut U) -> Option<Vec<T>>,
    additional_action_input: &mut U,
) -> NestedNode<R>
where
    T: Clone,
    R: Clone,
{
    let action_result = action(current.clone(), additional_action_input);
    let mut nested_results = None;

    if let Some(children) = nested_reference(current.clone(), additional_action_input) {
        let mut result = vec![];

        for child in children {
            let child_result = convert_to_nested_node(
                child.clone(),
                action,
                nested_reference,
                additional_action_input,
            );

            result.push(child_result);
        }

        nested_results = match result.len().gt(&0) {
            true => Some(result),
            false => None,
        }
    }

    NestedNode {
        current: action_result,
        children: nested_results,
    }
}

/// Traverses a nested hierachy of [NestedNode]s and processes them.
pub fn traverse_nested_node<T, R, U>(
    current: NestedNode<T>,
    action: fn(NestedNode<T>, &mut U) -> R,
    nested_action: fn(child: NestedNode<T>, parent_result: &mut R, addition_action_input: &mut U),
    after_children_action: fn(parent_action_result: R, addition_action_input: &mut U),
    additional_action_input: &mut U,
) where
    T: Clone,
    R: Clone,
{
    let mut action_result = action(current.clone(), additional_action_input);

    if let Some(children) = current.clone().children {
        for child in children {
            nested_action(child.clone(), &mut action_result, additional_action_input);
            traverse_nested_node(
                child.clone(),
                action,
                nested_action,
                after_children_action,
                additional_action_input,
            )
        }

        after_children_action(action_result, additional_action_input);
    }
}

/// Maps a [NestedNode] to a different [NestedNode].
pub fn map_nested_node<T, R, U>(
    current: NestedNode<T>,
    action: fn(NestedNode<T>, &mut U) -> R,
    after_children_action: fn(parent_action_result: R, addition_action_input: &mut U),
    additional_action_input: &mut U,
) -> NestedNode<R>
where
    T: Clone,
    R: Clone,
{
    let action_result = action(current.clone(), additional_action_input);
    let mut nested_results = None;

    if let Some(children) = current.clone().children {
        let mut result = vec![];

        for child in children {
            let child_result = map_nested_node(
                child.clone(),
                action,
                after_children_action,
                additional_action_input,
            );

            result.push(child_result);
        }

        after_children_action(action_result.clone(), additional_action_input);

        nested_results = match result.len().gt(&0) {
            true => Some(result),
            false => None,
        }
    }

    NestedNode {
        current: action_result,
        children: nested_results,
    }
}
