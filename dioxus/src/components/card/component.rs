use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/card/style.css")]
struct Styles;

#[component]
pub fn Card(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card,
        "data-slot": "card",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}

#[component]
pub fn CardHeader(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_header,
        "data-slot": "card-header",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}

#[component]
pub fn CardTitle(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_title,
        "data-slot": "card-title",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}

#[component]
pub fn CardDescription(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_description,
        "data-slot": "card-description",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}

#[component]
pub fn CardAction(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_action,
        "data-slot": "card-action",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}

#[component]
pub fn CardContent(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_content,
        "data-slot": "card-content",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}

#[component]
pub fn CardFooter(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_card_footer,
        "data-slot": "card-footer",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div {
            ..merged,
            {children}
        }
    }
}
