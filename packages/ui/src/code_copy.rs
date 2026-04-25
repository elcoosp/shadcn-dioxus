use dioxus::prelude::*;

/// Wraps children and injects a "Copy" button on every <pre> element inside.
#[component]
pub fn CodeCopyWrapper(children: Element) -> Element {
    // Run after each render to attach copy buttons to new code blocks
    use_effect(move || {
        // Use a small delay so the DOM is fully built
        spawn(async move {
            // We need to wait for the next frame
            // But Dioxus's spawn already yields; still we add a tiny delay.
            futures_timer::Delay::new(std::time::Duration::from_millis(50)).await;
            // Evaluate JavaScript to add copy buttons to all pre elements that don't have one yet.
            // This uses the document::eval function available in Dioxus web.
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
        });
    });

    rsx! {
        div { class: "doc-content", {children} }
    }
}
