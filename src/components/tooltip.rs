//! Tooltip
use crate::{
    prelude::{ExtendClasses, Orientation},
    utils::popper::*,
};
use yew::prelude::*;

/// Properties for [`Tooltip`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TooltipProperties {
    pub children: Children,
    pub text: String,
}

/// Tooltip component
///
/// > A **tooltip** is in-app messaging used to identify elements on a page with short, clarifying text.
///
/// See: <https://www.patternfly.org/v4/components/tooltip>
///
/// ## Properties
///
/// Defined by [`TooltipProperties`].
#[function_component(Tooltip)]
pub fn view(props: &TooltipProperties) -> Html {
    let active = use_state_eq(|| false);
    let state = use_state_eq(|| Option::<PopperState>::None);

    let onmouseenter = {
        let active = active.clone();
        Callback::from(move |_| {
            log::debug!("OnMouseEnter");
            active.set(true);
        })
    };
    let onmouseleave = {
        let active = active.clone();
        Callback::from(move |_| {
            log::debug!("OnMouseLeave");
            active.set(false);
        })
    };

    let content_ref = use_node_ref();
    let target_ref = use_node_ref();

    let content = html!(
        <TooltipPopupContent
            state={(*state).clone()}
            text={props.text.clone()}
            r#ref={content_ref.clone()}
        />
    );

    let onstatechange = {
        let state = state.clone();
        use_memo(
            move |()| {
                let state = state.clone();
                Callback::from(move |new_state| {
                    state.set(Some(new_state));
                })
            },
            (),
        )
    };

    let options = PopperOptions {
        strategy: PopperStrategy::Fixed,
        placement: PopperPlacement::Right,
        modifiers: vec![
            Modifier::Offset(Offset {
                skidding: 0,
                distance: 11,
            }),
            Modifier::PreventOverflow(PreventOverflow { padding: 0 }),
        ],
    };

    html! (
        <>
            <span {onmouseenter} {onmouseleave} ref={target_ref.clone()}>
                { for props.children.iter() }
            </span>
            <Popper
                visible={*active}
                {content}
                {content_ref}
                {target_ref}
                mode={PopperMode::Portal}
                onstatechange={(*onstatechange).clone()}
                {options}
            />
        </>
    )
}

#[derive(PartialEq, Properties)]
struct TooltipPopupContentProperties {
    text: String,
    state: Option<PopperState>,
    r#ref: NodeRef,
}

#[function_component(TooltipPopupContent)]
fn tooltip_popup_content(props: &TooltipPopupContentProperties) -> Html {
    let style = props
        .state
        .as_ref()
        .map(|s| s.styles.to_string())
        .unwrap_or_default();
    let orientation = props
        .state
        .as_ref()
        .map(|s| s.orientation)
        .unwrap_or(Orientation::Bottom);

    html! {
        <TooltipPopup
            r#ref={props.r#ref.clone()}
            {style}
            hidden={props.state.is_none()}
            orientation={orientation}
            text={props.text.clone()}
        />
    }
}

/// Properties for [`TooltipPopup`]
#[derive(Clone, PartialEq, Properties)]
pub struct TooltipPopupProperties {
    pub text: String,
    pub orientation: Orientation,
    #[prop_or_default]
    pub hidden: bool,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub r#ref: NodeRef,
}

/// The content shown when the tooltip pops up.
///
/// ## Properties
///
/// Defined by [`TooltipPopupProperties`].
#[function_component(TooltipPopup)]
pub fn tooltip_popup(props: &TooltipPopupProperties) -> Html {
    let mut class = Classes::from("pf-v5-c-tooltip");

    class.extend_from(&props.orientation);

    let style = if props.hidden {
        "display: none;"
    } else {
        &props.style
    }
    .to_string();

    html! {
        <div ref={&props.r#ref} {style} {class} role="tooltip">
            <div class="pf-v5-c-tooltip__arrow"></div>
            <div class="pf-v5-c-tooltip__content">
                { &props.text }
            </div>
        </div>
    }
}
