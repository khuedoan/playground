use crate::components::{
    button::{ButtonSize, ButtonVariant, button_variants},
    dropdown_menu::{
        dropdown_menu, dropdown_menu_content, dropdown_menu_item, dropdown_menu_label,
        dropdown_menu_separator, dropdown_menu_trigger,
    },
};
use topcoat::{
    Result,
    font::fontsource::fontsource_font,
    icon::{icon, iconify::iconify_icon},
    router::{Slot, layout},
    tailwind,
    view::{attributes, view},
};

#[layout("/")]
pub async fn root_layout(slot: Slot<'_>) -> Result {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"Netamos"</title>
                topcoat::font::link(
                    font: fontsource_font!(
                        GEIST,
                        weight: [400, 500, 600],
                        style: Normal,
                        host: Asset,
                    )
                )
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                topcoat::dev::script()
            </head>
            <body>(slot.await?)</body>
        </html>
    }
}

#[layout("/tenants")]
pub async fn app_layout(slot: Slot<'_>) -> Result {
    view! {
        <header class="border-b border-border">
            <nav class="mx-auto flex h-16 max-w-6xl items-center gap-2 px-4 sm:px-6" aria-label="Global">
                <a href="/tenants/default" class="mr-auto flex items-center gap-2 font-semibold">
                    icon(data: iconify_icon!("feather:hexagon"), attrs: attributes! { class="size-6" })
                    "Netamos"
                </a>
                <a
                    href="/tenants/default"
                    class=(button_variants(ButtonVariant::Ghost, ButtonSize::Sm))
                >
                    "Home"
                </a>
                <a
                    href="/tenants/default/settings"
                    class=(button_variants(ButtonVariant::Ghost, ButtonSize::Sm))
                >
                    "Settings"
                </a>
                dropdown_menu(
                    dropdown_menu_trigger(
                        attrs: attributes! {
                            class=(button_variants(ButtonVariant::Outline, ButtonSize::Sm))
                        },
                        icon(data: iconify_icon!("feather:layers"), attrs: attributes! { class="size-4" })
                        <span class="hidden sm:inline">"Netamos"</span>
                        icon(data: iconify_icon!("feather:chevron-down"), attrs: attributes! { class="size-3" })
                    )
                    dropdown_menu_content(
                        attrs: attributes! { class="right-0 left-auto" },
                        dropdown_menu_label("Tenant")
                        <form action="/tenants/default" method="get">
                            dropdown_menu_item(
                                attrs: attributes! { type="submit" },
                                icon(data: iconify_icon!("feather:check"), attrs: attributes! { class="size-4" })
                                "Netamos"
                            )
                        </form>
                        dropdown_menu_separator()
                        <form action="/tenants/new" method="get">
                            dropdown_menu_item(
                                attrs: attributes! { type="submit" },
                                icon(data: iconify_icon!("feather:plus"), attrs: attributes! { class="size-4" })
                                "Create tenant"
                            )
                        </form>
                        <form action="/login" method="get">
                            dropdown_menu_item(
                                attrs: attributes! { type="submit" },
                                icon(data: iconify_icon!("feather:log-out"), attrs: attributes! { class="size-4" })
                                "Sign out"
                            )
                        </form>
                    )
                )
            </nav>
        </header>
        (slot.await?)
    }
}
