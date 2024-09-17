use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct MessageProps {
    color: Option<String>,
    text: String,
}

#[component]
pub fn SimpleMessage(props: MessageProps) -> Element {
    rsx! {
        p {
            i {
                color: props.color.unwrap_or_else(|| "gray".to_string()),
                "{props.text}"
            }
        }
    }
}
