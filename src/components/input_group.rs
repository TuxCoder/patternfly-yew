//! Input group

use crate::prelude::ChildrenProperties;
use yew::prelude::*;

/// Properties for [`InputGroup`]
#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupProperties {
    pub children: Children,

    #[prop_or_default]
    pub plain: bool,

    #[prop_or_default]
    pub aria_label: AttrValue,
}

/// Input group component
///
/// > An **input group** groups multiple related controls or inputs together so they appear as one control.
///
/// See: <https://www.patternfly.org/v4/components/input-group>
///
/// ## Properties
///
/// Defined in [`InputGroupProperties`].
///
/// ## Children
///
/// Input groups can have form elements as their children, and also make use of the
/// [`InputGroupItem`] component to amend the input group with additional content.
#[function_component(InputGroup)]
pub fn input_group(props: &InputGroupProperties) -> Html {
    let mut class = classes!("pf-v5-c-input-group");

    if props.plain {
        class.push(classes!("pf-m-plain"));
    }

    html! (
        <div {class} aria-label={&props.aria_label}>
            { for props.children.iter() }
        </div>
    )
}

/// Input group text, as child of [`InputGroupItem`]
#[function_component(InputGroupText)]
pub fn input_group_text(props: &ChildrenProperties) -> Html {
    html!(
        <span class={"pf-v5-c-input-group__text"}>
            { for props.children.iter() }
        </span>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct InputGroupItemProperties {
    /// The element's ID
    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub plain: bool,

    #[prop_or_default]
    pub fill: bool,

    #[prop_or_default]
    pub r#box: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub children: Children,
}

/// Additional item on an input group.
#[function_component(InputGroupItem)]
pub fn input_group_item(props: &InputGroupItemProperties) -> Html {
    let mut class = classes!("pf-v5-c-input-group__item");

    if props.plain {
        class.push(classes!("pf-m-plain"));
    }

    if props.fill {
        class.push(classes!("pf-m-fill"));
    }

    if props.r#box {
        class.push(classes!("pf-m-box"));
    }

    if props.disabled {
        class.push(classes!("pf-m-disabled"));
    }

    html!(
        <div {class} id={&props.id}>
            { for props.children.iter() }
        </div>
    )
}
