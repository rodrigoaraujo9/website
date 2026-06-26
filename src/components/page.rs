use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx!(
        div {
            class:"page",
            div {
                class: "terminal",

                header {
                    b { "rodrigoaraujo" }
                    " ~/home"
                }

                div {
                    class: "ls",
                    "> $ ls "
                }

                // add time in braga

                nav {
                    class: "nav",
                    ul {
                        li { Link { to: Route::About {}, "about" } }
                        li { Link { to: Route::Work {}, "work" } }
                        li { Link { to: Route::Music {}, "music" } }
                        li { Link { to: Route::Photos {}, "photos" } }
                    }
                }

                div {
                    class: "ls",
                    "> $ "
                    span {
                        class: "cursor",
                        "█"
                    }
                }
            }
        }
        // div {
        //     "Hi! I'm Rodrigo Araújo and I am a CS Master's Student."
        // }
    )
}

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "page",
            div { class: "terminal-about",
                header {
                    b { "rodrigoaraujo" }
                    " ./about"
                }

                div { class: "ls", "> $ cat about.txt" }

                p {
                    "My name is Rodrigo Araújo and I'm from Braga, Portugal. I'm currently pursuing a Master's in Computer Science. My main areas of interest right now are programming languages and distributed systems. I have also quite the knack for systems programming so I've also made two synths recently - one in C with a funky physical controler and a TUI one in Rust. Computer Science is a pashion of mine, but so is composing and writing music — you could call it my creative outlet. I am currently starting my Master's thesis on combining cryptography with session types to enforce communication-security guarantees at compile-time."
                }

                div { class: "ls",
                    "> $ "
                        Link {
                            to: Route::Home {},
                            class: "cmd-link",
                            "cd .."
                        }
                    span { class: "cursor", "█" }
                }
            }
        }
    }
}

#[component]
pub fn Work() -> Element {
    rsx! {
        div { class: "page",
            div { class: "terminal",
                header {
                    b { "rodrigoaraujo" }
                    " ./work"
                }

                div { class: "ls", "> $ ls work" }

                p { "Projects coming soon." }

                div { class: "ls",
                    "> $ "
                        Link {
                            to: Route::Home {},
                            class: "cmd-link",
                            "cd .."
                        }
                    span { class: "cursor", "█" }
                }
            }
        }
    }
}

#[component]
pub fn Music() -> Element {
    rsx! {
        div { class: "page",
            div { class: "terminal",
                header {
                    b { "rodrigoaraujo" }
                    " ./music"
                }

                div { class: "ls", "> $ ls music" }

                p { "Music page coming soon." }

                div { class: "ls",
                    "> $ "
                        Link {
                            to: Route::Home {},
                            class: "cmd-link",
                            "cd .."
                        }
                    span { class: "cursor", "█" }
                }
            }
        }
    }
}

#[component]
pub fn Photos() -> Element {
    rsx! {
        div { class: "page",
            div { class: "terminal",
                header {
                    b { "rodrigoaraujo" }
                    " ./photos"
                }

                div { class: "ls", "> $ ls photos" }

                p { "Photos page coming soon." }

                div { class: "ls",
                    "> $ "
                        Link {
                            to: Route::Home {},
                            class: "cmd-link",
                            "cd .."
                        }
                    span { class: "cursor", "█" }
                }
            }
        }
    }
}
