use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct LinesProps {
    content: Vec<String>,
    font_size: u32,
}

#[component]
pub fn MultiLine(props: LinesProps) -> Element {
    rsx! {
        ul {
            class: "part",
            style: format!("font-size: {}px;", props.font_size),
            {props.content.into_iter().map(|line| rsx! {
                li { {line} }
            })}
        }
    }
}
