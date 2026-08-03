use dioxus::prelude::*;

use crate::Route;

// static GARGANTUA: Asset = asset!("/assets/images/gargantua.png");
// static SYNTHC: Asset = asset!("/assets/images/synth-c.jpeg");
static CV: Asset = asset!("/assets/rodrigoaraujo.pdf");

#[component]
pub fn Home() -> Element {
    rsx!(
        div {
            class:"page",
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

                // add time in braga

                nav {
                    class: "nav",
                    ul {
                        li { Link { to: Route::About {}, "about" } }
                        li { Link { to: Route::Projects {}, "projects" } }
                        li { Link { to: Route::Social {}, "social"} }
                        a {
                            href: "{CV}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "cv"
                        }
                        // li { Link { to: Route::Music {}, "music" } }
                        // li { Link { to: Route::Photos {}, "photos" } }
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
    )
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
                    "Hi! My name is Rodrigo Araújo and I'm from Braga, Portugal. As of this moment, I'm pursuing a Master's Degree in Computer Science and focusing my studies on programming languages and distributed systems. I have also got quite the knack for low-level systems programming so I've been diving into making synthesizers lately. I develop most of my work in Rust, Haskell and C."
                }
                p {
                    "Although computer science is one of my biggest devotions, I also compose, write and overall love music."
                }
                p {
                    "As for my academic endevours, I am currently starting my Master's thesis on combining cryptography with session types to enforce communication-security guarantees at compile-time."
                }

                // ls (education, music, maths?, background?)

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
pub fn Projects() -> Element {
    rsx! {
        div { class: "page",
            div { class: "work",
                header {
                    b { "rodrigoaraujo" }
                    " ~/projects"
                }
                div { class: "ls", "> $ cargo run" }

                div { class: "project-grid",

                    a {
                        class: "project-card",
                        href: "https://github.com/rodrigoaraujo9/synth-c",
                        target: "_blank",
                        rel: "noopener noreferrer",

                        div { class: "project-card-top",
                            div { class: "project-info",
                                h3 { "synth-c" }
                                p {
                                    "A real-time synth written in C with a physical controller. Real-time audio with multiple waveforms, filters and modulators. Also supports streaming audio to Android."
                                }
                            }
                        }
                    }

                    a {
                        class: "project-card",
                        href: "https://github.com/rodrigoaraujo9/gargantua",
                        target: "_blank",
                        rel: "noopener noreferrer",

                        div { class: "project-card-top",
                            div { class: "project-info",
                                h3 { "gargantua" }
                                p {
                                    "A real-time Rust simulation of light orbiting a Schwarzschild black hole. Made with Raylib and it is currently 2D."
                                }
                            }
                        }
                    }

                    a {
                        class: "project-card",
                        href: "https://github.com/rodrigoaraujo9/blocktion",
                        target: "_blank",
                        rel: "noopener noreferrer",

                        div { class: "project-card-top",
                            div { class: "project-info",
                                h3 { "blocktion" }
                                p {
                                    "A secure proof-of-work blockchain for auctions implemented in Rust from scratch. This is the biggest project I have worked on."
                                }
                            }
                        }
                    }

                    a {
                        class: "project-card",
                        href: "https://github.com/rodrigoaraujo9/garbage-collector",
                        target: "_blank",
                        rel: "noopener noreferrer",

                        div { class: "project-card-top",
                            div { class: "project-info",
                                h3 { "garbage-collector" }
                                p {
                                    "A collection of garbage collector implementations with focus on optimization and experimentation."
                                }
                            }
                        }
                    }

                    a {
                        class: "project-card",
                        href: "https://github.com/rodrigoaraujo9/mugen",
                        target: "_blank",
                        rel: "noopener noreferrer",

                        div { class: "project-card-top",
                            div { class: "project-info",
                                h3 { "mugen" }
                                p {
                                    "A real-time, terminal-based synthesizer written in Rust. It supports multiple waveforms and effects and has an easily extendible modular audio chain."
                                }
                            }
                        }
                    }

                    a {
                        class: "project-card",
                        href: "https://github.com/rodrigoaraujo9/sisyphus-retrojam-2025",
                        target: "_blank",
                        rel: "noopener noreferrer",

                        div { class: "project-card-top",
                            div { class: "project-info",
                                h3 { "sysyphus" }
                                p {
                                    "A retro platformer game developed for IEEE RetroJam 2025 using Raylib and Rust about the themes of rebirth. I also personally produced the soundtrack and ambience."
                                }
                            }
                        }
                    }

                    a {
                        class: "project-card",
                        href: "https://github.com/rodrigoaraujo9/lambda-calculus",
                        target: "_blank",
                        rel: "noopener noreferrer",

                        div { class: "project-card-top",
                            div { class: "project-info",
                                h3 { "lambda-calculus" }
                                p {
                                    "A collection of lambda-calculus interpreters and compilers to SECD and Extended SKI written in Haskell, with a Happy parser."
                                }
                            }
                        }
                    }
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
pub fn Social() -> Element {
    rsx! {
        div { class: "page",
            div { class: "terminal",
                header {
                    b { "rodrigoaraujo" }
                    " ~/social"
                }

                div { class: "ls", "> $ ls" }

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

                        li {
                            a {
                                href: "https://letterboxd.com/rodrigoaraujo9/",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "letterboxd"
                            }
                        }

                        li {
                            a {
                                href: "https://open.spotify.com/user/11161909394?si=4b52211aa39046f8",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "spotify"
                            }
                        }
                    }
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
pub fn Music() -> Element {
    rsx! {
        div { class: "page",
            div { class: "terminal",
                header {
                    b { "rodrigoaraujo" }
                    " ~/music"
                }

                div { class: "ls", "> $ cat README.txt" }

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
                    " ~/photos"
                }

                div { class: "ls", "> $ cat README.txt" }

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
