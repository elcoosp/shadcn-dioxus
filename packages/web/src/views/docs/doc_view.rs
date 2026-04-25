use crate::docs::ParsedDoc;
use crate::views::docs::utils::create_doc_components;
use crate::Route;
use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use ui::ButtonVariant;

#[component]
pub fn DocView(parsed_content: Option<ParsedDoc>) -> Element {
    let custom_components = create_doc_components();

    match parsed_content {
        Some(parsed) => {
            rsx! {
                div { class: "flex flex-col gap-8",
                    div { class: "flex flex-col gap-2",
                        div { class: "flex items-start justify-between",
                            h1 { class: "scroll-m-20 text-4xl font-semibold tracking-tight sm:text-3xl xl:text-4xl",
                                "{parsed.frontmatter.title}"
                            }
                        }
                        p { class: "text-muted-foreground text-balance text-[1.05rem] sm:text-base",
                            "{parsed.frontmatter.description}"
                        }
                        div {
                            class: "doc-content mt-12 space-y-4 [&>*>pre]:p-4 [&>*>pre]:rounded-md [&>*>pre]:bg-code! [&>*>pre]:text-code-foreground [&>pre]:overflow-x-auto",
                            onmounted: move |_| {
                                // Add copy buttons to all <pre> elements once the content is in the DOM
                                document::eval(
                                    r#"
                                    (function() {
                                        document.querySelectorAll('.doc-content pre').forEach(function(pre) {
                                            if (pre.querySelector('.copy-btn')) return;
                                            var btn = document.createElement('button');
                                            btn.className = 'copy-btn';
                                            btn.textContent = 'Copy';
                                            btn.addEventListener('click', function() {
                                                var code = pre.querySelector('code') || pre;
                                                navigator.clipboard.writeText(code.textContent).then(function() {
                                                    btn.textContent = 'Copied!';
                                                    setTimeout(function() { btn.textContent = 'Copy'; }, 2000);
                                                });
                                            });
                                            pre.style.position = 'relative';
                                            pre.appendChild(btn);
                                        });
                                    })();
                                    "#
                                );
                            },
                            Markdown {
                                src: parsed.content.clone(),
                                components: custom_components,
                                theme: "base16-ocean.dark",
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                div { class: "container mx-auto flex-1 py-12 text-center",
                    h1 { class: "text-4xl font-bold mb-4", "404" }
                    p { class: "text-muted-foreground mb-6", "No documentation found at this path." }
                    Link {
                        class: ui::button_variants(ButtonVariant::Default, ui::ButtonSize::Default),
                        to: Route::Home {},
                        "Go Home"
                    }
                }
            }
        }
    }
}
