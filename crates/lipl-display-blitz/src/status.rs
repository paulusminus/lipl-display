use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct StatusProps {
    font_size: u32,
    text: String,
}

#[component]
pub fn Status(props: StatusProps) -> Element {
    rsx! {
        p {
            class: "status",
            style: format!("font-size: {}px;", props.font_size),
            span {
                display: "block",
                style: format!("font-size: {}px;", props.font_size.saturating_sub(2)),
                {props.text}
            }
        }
    }
}
