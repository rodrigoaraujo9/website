use crate::Route;
use dioxus::prelude::*;
use gloo_timers::future::sleep;
use std::time::Duration;

// static GARGANTUA: Asset = asset!("/assets/images/gargantua.png");
// static SYNTHC: Asset = asset!("/assets/images/synth-c.jpeg");
static CV: Asset = asset!("/assets/rodrigoaraujo.pdf");

#[component]
pub fn Home() -> Element {
    let mut prompt = use_signal(String::new);
    let mut typing = use_signal(|| false);

    use_future(move || async move {
        const TEXT: &str = "click to interact";

        let char_count = TEXT.chars().count();

        loop {
            // before typing
            sleep(Duration::from_millis(2000)).await;

            typing.set(true);

            for i in 1..=char_count {
                let text = TEXT.chars().take(i).collect::<String>();
                prompt.set(text);

                // irregular typing
                let delay = match i % 5 {
                    0 => 110,
                    1 => 55,
                    2 => 80,
                    3 => 40,
                    _ => 65,
                };

                sleep(Duration::from_millis(delay)).await;
            }

            typing.set(false);

            // after typing and before deleting
            sleep(Duration::from_millis(2200)).await;

            typing.set(true);

            for i in (0..char_count).rev() {
                let text = TEXT.chars().take(i).collect::<String>();
                prompt.set(text);

                // deleting is faster and more regular
                let delay = match i % 4 {
                    0 => 70,
                    1 => 45,
                    2 => 60,
                    _ => 50,
                };

                sleep(Duration::from_millis(delay)).await;
            }

            typing.set(false);

            // before repeating
            sleep(Duration::from_millis(15000)).await;
        }
    });

    rsx! {
        div {
            class: "page",

            div {
                class: "terminal",

                header {
                    b { "rodrigoaraujo" }
                    " ~/"
                }

                div {
                    class: "ls",
                    "> $ ls "
                }

                nav {
                    class: "nav",

                    ul {
                        li {
                            Link {
                                to: Route::About {},
                                "about"
                            }
                        }

                        li {
                            Link {
                                to: Route::Projects {},
                                "projects"
                            }
                        }

                        li {
                            Link {
                                to: Route::Social {},
                                "social"
                            }
                        }

                        a {
                            href: "{CV}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "cv"
                        }

                        // li {
                        //     Link {
                        //         to: Route::Music {},
                        //         "music"
                        //     }
                        // }

                        // li {
                        //     Link {
                        //         to: Route::Photos {},
                        //         "photos"
                        //     }
                        // }
                    }
                }

                div {
                    class: "ls interactive-prompt",

                    onclick: move |_| {
                        prompt.set(String::new());
                    },

                    "> $ "

                    span {
                        "{prompt}"
                    }

                    span {
                        class: if typing() {
                            "cursor typing"
                        } else {
                            "cursor"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn About() -> Element {
    rsx! {
    div { class: "page",
        div { class: "about",
            header {
                b { "rodrigoaraujo" }
                " ~/about"
            }

            div { class: "ls", "> $ cat about.txt" }

            p {

                "Hi! My name is "
                span { class: "highlight", "Rodrigo Araújo" }
                " and I'm from Braga, "
                span { class: "highlight", "Portugal" }
                ". As of this moment, I'm pursuing a "
                span { class: "highlight", "Master's" }
                " degree in "
                span { class: "highlight", "Computer Science" }
                " at FCUP"

                " and focusing my studies on "
                span { class: "highlight", "programming languages" }
                " and "
                span { class: "highlight", "distributed systems" }
                ". I have also got quite the knack for "
                span { class: "highlight", "low-level systems programming" }
                " so I've made two synthesizers this past semester. I develop most of my work in "
                span { class: "highlight", "Rust" }
                ", "
                span { class: "highlight", "Haskell" }
                " and "
                span { class: "highlight", "C" }
                "."
            }

            p {
                "Although CS is one of my biggest devotions, I also "
                span { class: "highlight", "compose" }
                ", "
                span { class: "highlight", "write" }
                " and overall love "
                span { class: "highlight", "music" }
                "."
            }

            p {
                "As for my academic endeavours, I am currently starting my Master's thesis on combining "
                span { class: "highlight", "cryptography" }
                " with "
                span { class: "highlight", "context-free session types" }
                " to enforce communication-security guarantees at compile-time."
            }

            // ls (education, music, maths?, background?)

                "> $ "

                Link {
                    to: Route::Home {},
                    class: "cmd-link",
                    "cd .."
                }

                span {
                    class: "cursor"
                }
            }
        }
    }
}

#[component]
pub fn Projects() -> Element {
    rsx! {
        div {
            class: "page",

            div {
                class: "work",

                header {
                    b { "rodrigoaraujo" }
                    " ~/projects"
                }

                div {
                    class: "ls",
                    "> $ cargo run --release"
                }

                div {
                    class: "project-grid",

                    a {
                        class: "project-card",
                        href: "https://github.com/rodrigoaraujo9/mugen",
                        target: "_blank",
                        rel: "noopener noreferrer",

                        span {
                            class: "project-name",
                            "mugen"
                        }

                        span {
                            class: "project-description",
                            "Terminal synthesizer with modular DSP."
                        }
                    }

                    a {
                        class: "project-card",
                        href: "https://github.com/rodrigoaraujo9/gargantua",
                        target: "_blank",
                        rel: "noopener noreferrer",

                        span {
                            class: "project-name",
                            "gargantua"
                        }

                        span {
                            class: "project-description",
                            "Schwarzschild black-hole simulator in Rust."
                        }
                    }

                    a {
                        class: "project-card",
                        href: "https://github.com/rodrigoaraujo9/blocktion",
                        target: "_blank",
                        rel: "noopener noreferrer",

                        span {
                            class: "project-name",
                            "blocktion"
                        }

                        span {
                            class: "project-description",
                            "Proof-of-work auction blockchain in Rust."
                        }
                    }

                    a {
                        class: "project-card",
                        href: "https://github.com/rodrigoaraujo9/synth-c",
                        target: "_blank",
                        rel: "noopener noreferrer",

                        span {
                            class: "project-name",
                            "synth-c"
                        }

                        span {
                            class: "project-description",
                            "Real-time synthesizer written in C."
                        }
                    }

                    a {
                        class: "project-card",
                        href: "https://github.com/rodrigoaraujo9/sisyphus-retrojam-2025",
                        target: "_blank",
                        rel: "noopener noreferrer",

                        span {
                            class: "project-name",
                            "sisyphus"
                        }

                        span {
                            class: "project-description",
                            "Retro platformer made for IEEE RetroJam 2025."
                        }
                    }

                    a {
                        class: "project-card",
                        href: "https://github.com/rodrigoaraujo9/garbage-collector",
                        target: "_blank",
                        rel: "noopener noreferrer",

                        span {
                            class: "project-name",
                            "garbage-collector"
                        }

                        span {
                            class: "project-description",
                            "Experimental garbage collector implementations."
                        }
                    }

                    a {
                        class: "project-card",
                        href: "https://github.com/rodrigoaraujo9/lambda-calculus",
                        target: "_blank",
                        rel: "noopener noreferrer",

                        span {
                            class: "project-name",
                            "lambda-calculus"
                        }

                        span {
                            class: "project-description",
                            "Lambda calculus interpreters and compilers."
                        }
                    }
                }

                div {
                    class: "ls",
                    "> $ "

                    Link {
                        to: Route::Home {},
                        class: "cmd-link",
                        "cd .."
                    }

                    span {
                        class: "cursor"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Social() -> Element {
    rsx! {
        div {
            class: "page",

            div {
                class: "terminal",

                header {
                    b { "rodrigoaraujo" }
                    " ~/social"
                }

                div {
                    class: "ls",
                    "> $ ls"
                }

                nav {
                    class: "nav2",
                    ul {
                        li {
                            a {
                                href: "https://github.com/rodrigoaraujo9",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "github"
                            }
                        }

                        li {
                            a {
                                href: "https://www.linkedin.com/in/rodrigoaraujo9/",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "linkedin"
                            }
                        }

                        li {
                            a {
                                href: "mailto:contact@rodrigoaraujo.pt",
                                "email"
                            }
                        }

                        // li {
                        //     a {
                        //         href: "https://letterboxd.com/rodrigoaraujo9/",
                        //         target: "_blank",
                        //         rel: "noopener noreferrer",
                        //         "letterboxd"
                        //     }
                        // }

                        // Link {
                        //     to: Route::Music {},
                        //     class: "cmd-link",
                        //     "music"
                        // }

                        // li {
                        //     a {
                        //         href: "https://open.spotify.com/user/11161909394?si=4b52211aa39046f8",
                        //         target: "_blank",
                        //         rel: "noopener noreferrer",
                        //         "spotify"
                        //     }
                        // }
                    }
                }

                div {
                    class: "ls",

                    "> $ "

                    Link {
                        to: Route::Home {},
                        class: "cmd-link",
                        "cd .."
                    }

                    span {
                        class: "cursor"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Music() -> Element {
    rsx! {
        div {
            class: "page",

            div {
                class: "terminal",

                header {
                    b { "rodrigoaraujo" }
                    " ~/music"
                }

                div {
                    class: "ls",
                    "> $ cat README.txt"
                }

                p {
                    "Music page coming soon."
                }

                div {
                    class: "ls",

                    "> $ "

                    Link {
                        to: Route::Social {},
                        class: "cmd-link",
                        "cd .."
                    }

                    span {
                        class: "cursor"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Photos() -> Element {
    rsx! {
        div {
            class: "page",

            div {
                class: "terminal",

                header {
                    b { "rodrigoaraujo" }
                    " ~/photos"
                }

                div {
                    class: "ls",
                    "> $ cat README.txt"
                }

                p {
                    "Photos page coming soon."
                }

                div {
                    class: "ls",

                    "> $ "

                    Link {
                        to: Route::Home {},
                        class: "cmd-link",
                        "cd .."
                    }

                    span {
                        class: "cursor"
                    }
                }
            }
        }
    }
}
