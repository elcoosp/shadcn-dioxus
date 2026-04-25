use crate::components::{FullLayout, Hero, SidebarLayout};
use dioxus::prelude::*;
use ui::{ButtonVariant, PortalProvider};
use views::{ComponentDoc, ComponentView, Home, InstallationView, PlaygroundExample, ThemingView};
mod components;
mod demos;
mod docs;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/examples/playground")]
    PlaygroundExample {},
    #[route("/docs/installation")]
    InstallationView {},
    #[route("/docs/theming")]
    ThemingView {},
    #[route("/docs/components")]
    ComponentView {},
    #[route("/docs/components/:name")]
    ComponentDoc { name: String },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css", CssAssetOptions::new());
const _: Asset = asset!("/assets/og.png", AssetOptions::image().with_hash_suffix(false));
const _: Asset = asset!("/assets/web-app-manifest-192x192.png", AssetOptions::image().with_hash_suffix(false));
const _: Asset = asset!("/assets/web-app-manifest-512x512.png", AssetOptions::image().with_hash_suffix(false));

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "The Component Library for Dioxus" }
        document::Meta { charset: "utf-8" }
        document::Meta { name: "description", content: "A set of beautifully designed components that you can customize, extend, and build on." }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Meta { property: "og:title", content: "The Component Library for Dioxus" }
        document::Meta { property: "og:description", content: "A set of beautifully designed components that you can customize, extend, and build on." }
        document::Meta { property: "og:image", content: "https://shadcn-dioxus.com/assets/og.png" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:image", content: "https://shadcn-dioxus.com/assets/og.png" }
        document::Meta { name: "apple-mobile-web-app-title", content: "Shadcn Dioxus" }
        document::Link { rel: "icon", r#type: "image/png", href: "/favicon-96x96.png", sizes: "96x96" }
        document::Link { rel: "icon", r#type: "image/svg+xml", href: "/favicon.svg" }
        document::Link { rel: "shortcut icon", href: FAVICON }
        document::Link { rel: "apple-touch-icon", sizes: "180x180", href: "/apple-touch-icon.png" }
        document::Link { rel: "manifest", href: "/site.webmanifest" }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        PortalProvider {
            Router::<Route> {}
        }
    }
}

/// Top‑level layout that picks FullLayout or SidebarLayout automatically.
#[component]
fn WebNavbar() -> Element {
    let route = use_route::<Route>();

    let is_docs = matches!(&route,
        Route::ComponentDoc { .. } |
        Route::ComponentView {} |
        Route::InstallationView {} |
        Route::ThemingView {}
    );

    rsx! {
        div { class: "min-h-svh flex flex-col",
            components::Navbar {
                Link {
                    class: ui::button_variants(ButtonVariant::Ghost, ui::ButtonSize::Default),
                    to: Route::Home {},
                    "Home"
                }
                Link {
                    class: ui::button_variants(ButtonVariant::Ghost, ui::ButtonSize::Default),
                    to: Route::ComponentView {},
                    "Components"
                }
            }
            div { class: "grow",
                if is_docs {
                    SidebarLayout {
                        Outlet::<Route> {}
                    }
                } else {
                    FullLayout {
                        Outlet::<Route> {}
                    }
                }
            }
        }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    let path = route.join("/");
    rsx! {
        div { class: "container mx-auto flex-1 py-12 text-center",
            h1 { class: "text-4xl font-bold mb-4", "404" }
            p { class: "text-muted-foreground mb-6", "The page /{path} does not exist." }
            Link {
                class: ui::button_variants(ButtonVariant::Default, ui::ButtonSize::Default),
                to: Route::Home {},
                "Go to Home"
            }
        }
    }
}
