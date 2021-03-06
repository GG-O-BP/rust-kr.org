#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use maud::{html, Markup, PreEscaped, Render};
use ammonia;
use pulldown_cmark::{Parser, html};

fn head() -> Markup {
    html!{
        meta
            charset="UTF-8"
            http-equiv="X-UA-Compatible"
            name="viewport"
            content="IE-edge, width=device-width, initial-scale=1";
        title { "한국 러스트 사용자 그룹" }
        link
            rel="icon"
            href="/static/logo.svg"
            type="image/svg+xml";
        link
            rel="icon"
            href="/static/logo-512.png"
            type="image/png";
        link
            rel="stylesheet"
            href="https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css";
        link
            rel="stylesheet"
            href="/static/style.css";
    }
}

fn header() -> Markup {
    html!{
        a.unless-mobile.github-corner
            href="https://github.com/rust-kr/rust-kr.org"
            aria-label="View source on Github" {
                svg
                    width="80"
                    height="80"
                    viewBox="0 0 250 250"
                    aria-hidden="true" {
                        path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z" {}
                        path.octo-arm d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor" {}
                        path.octo-body d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z" fill="currentColor" {}
                    }
            }
        h1#header {
            a href="/" {
                img#logo
                    src ="/static/logo.svg"
                    alt ="Rust logo";
            }
            span#text {
                "한국 러스트 사용자 그룹"
                br;
                small {
                    a href="/" { "rust-kr.org" }
                }
            }
        }
        hr;
    }
}

fn footer() -> Markup {
    html!{
        hr;
        footer {
            div {
                a href="/pages/_pages" { "모든 문서 보기" }
                span.if-mobile {
                    a href="https://github.com/rust-kr/rust-kr.org" {
                        "사이트 소스 보기"
                    }
                }
            }
            div {
                "Powered by"
                a
                    style = "font-weight:700"
                    href = "https://rocket.rs/" {
                        "Rocket"
                    }
            }
        }
    }
}

#[get("/")]
fn index() -> Markup {
    html!{
        (head())
        (header())
        article {
            // ^Markdown::from_string(markdown)
        }
        (footer())
    }
}

// fn rocket() -> rocket::Rocket {
    // rocket::ignite().mount("/static", StaticFiles::from("static"))
// }
//
struct Markdown<T: AsRef<str>>(T);

impl<T: AsRef<str>> Render for Markdown<T> {
    fn render(&self) -> Markup {
        // Generate raw HTML
        let mut unsafe_html = String::new();
        let parser = Parser::new(self.0.as_ref());
        html::push_html(&mut unsafe_html, parser);
        // Sanitize it with ammonia
        let safe_html = ammonia::clean(&unsafe_html);
        PreEscaped(safe_html)
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from("static/static"))
        .launch();
}
