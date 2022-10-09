use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let dark = use_state(|| false);
    let onclick = {
        let d = dark.clone();
        Callback::from(move |_| d.set(!*d))
    };
    {
        let d = dark.clone();
        use_effect(move || {
            let e = gloo_utils::document_element();

            if *d {
                e.set_class_name("dark")
            } else {
                e.set_class_name("")
            }

            move || {
                e.set_class_name("");
            }
        })
    }

    html! {
        <header class="fixed z-10 w-screen bg-gray-500 dark:bg-gray-800">
        <nav class="mx-auto px-2 sm:px-6 lg:px-8 grid grid-cols-2 md:grid-cols-3 gap-2">
            <div class="relative flex items-center justify-start h-16 col-span-1 md:col-span-2">
                <a href="#introduction" class="text-black dark:text-white">{"Portfolio"}</a>
                <div class="form-check form-switch ml-4">
                    <input class="element-toggle" type="checkbox" role="switch" checked={*dark} {onclick} />
                </div>
                <img src="./asset/moon-svgrepo-com.svg" alt="Night mode"
                class="h-8 w-8 filter dark:invert ml-4" />
            </div>
            <ul class="relative hidden sm:flex items-center justify-end h-16 text-black dark:text-white">
                <li class="mr-3">{"Made With:"}</li>
                <li>
                    <a href="https://www.rust-lang.org/">
                        <img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg"
                        class="h-10 w-10 mr-3 filter dark:invert" alt="Rust" />
                    </a>
                </li>
                <li>
                    <a href="https://yew.rs/" >
                        <img src="https://yew.rs/img/logo.png"
                        class="h-10 w-10 mr-3" alt="Yew" />
                    </a>
                </li>
                <li>
                    <a href="https://tailwindcss.com/">
                        <img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Tailwind_CSS_Logo.svg"
                        class="h-10 w-10" alt="Tailwind CSS" />
                    </a>
                </li>
            </ul>
        </nav>
    </header>
    }
}

#[function_component(Introduction)]
pub fn introduction() -> Html {
    html! {
        <section id="introduction" class="section-commons px-8 sm:px-12 lg:px-32 grid grid-cols-2 md:grid-cols-3 content-center">
        <div class="flex flex-col justify-center content-start col-span-2">
            <p class="text-left mt-16 text-xl">
                <span class="motion-safe:animation fade-in text-3xl sm:text-5xl  md:text-7xl text-blue-600 dark:text-red-700">
                    {"Hello world! I'm Tech."}
                </span>
                <br />
                <span class="motion-safe:animation delay-1s fade-in md:text-4xl">
                    {"A multi-lingual, multi-disciplinary,"}
                    <br/>
                    {"and self-taught hobbyist developer."}
                </span>
            </p>
            <p class="motion-safe:animation delay-2s fade-in text-left mt-4 text-sm md:text-xl">
                {"I work on curious projects, with emphasis on discovery and understanding."}
                <br/>
                {"Currently making an effort to learn "} <a href="https://www.rust-lang.org/" class="text-blue-600 font-bold dark:text-red-700">{"Rust"}</a>{"."}
            </p>
            <div class="motion-safe:animation delay-3s fade-in self-start mt-8">
                <a class="btn-primary block motion-safe:animation delay-4s animate-bounce" href="https://github.com/TechTheAwesome">
                    {"Checkout my projects on github!"}
                </a>
            </div>
            <div class="self-center motion-safe:animation delay-3s fade-in mt-8">
                <a href="#technologies">
                    <img src="https://upload.wikimedia.org/wikipedia/commons/9/9d/Arrow-down.svg"
                        class="h-24 w-24" alt="scroll down" />
                </a>
            </div>
        </div>
    </section>
    }
}

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {}
}

#[function_component(Technologies)]
pub fn technologies() -> Html {
    html! {
        <section id="technologies" class="section-commons px-8 sm:px-12 lg:px-32 flex flex-col justify-center content-center text-center">
        <span class="text-2xl sm:text-4xl text-blue-600 dark:text-red-700">{"Technologies I love"}</span>
        <ul class="flex flex-row flex-wrap justify-center content-center animation fade-in delay-2s" >
            <li class="m-2">
                <a href="https://www.python.org/">
                    <img src="https://upload.wikimedia.org/wikipedia/commons/c/cf/Python_logo_51.svg"
                    class="h-24 w-24" alt="Python" />
                </a>
            </li>
            <li class="m-2">
                <a href="https://ipfs.io/">
                    <img src="https://upload.wikimedia.org/wikipedia/commons/1/18/Ipfs-logo-1024-ice-text.png"
                    class="h-24 w-24" alt="IPFS" />
                </a>
            </li>
            <li class="m-2 inline-block overflow-hidden">
                <a href="https://dotnet.microsoft.com/en-us/languages/csharp">
                    <img src="https://upload.wikimedia.org/wikipedia/commons/4/4f/Csharp_Logo.png" class="h-24 w-24 scale-150" alt="Csharp" />
                </a>
            </li>
            <li class="m-2">
                <a href="https://reactjs.org/" >
                    <img src="https://upload.wikimedia.org/wikipedia/commons/a/a7/React-icon.svg"
                    class="object-fit h-24 w-24 filter brightness-75 dark:brightness-100" alt="React" />
                </a>
            </li>
            <li class="m-2">
                <a href="https://www.typescriptlang.org/">
                    <img src="https://upload.wikimedia.org/wikipedia/commons/4/4c/Typescript_logo_2020.svg"
                    class="h-24 w-24" alt="Typescript" />
                </a>
            </li>
            <li class="m-2">
                <a href="https://www.rust-lang.org/">
                    <img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg"
                    class="h-24 w-24 filter dark:invert" alt="Rust" />
                </a>
            </li>
            <li class="m-2">
                <a href="https://tailwindcss.com/">
                    <img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Tailwind_CSS_Logo.svg"
                    class="h-24 w-24" alt="Tailwind CSS" />
                </a>
            </li>

        </ul>
        <span class="text-sm">{"Icons from "}<a href="https://www.wikipedia.org/">{"wikipedia"}</a></span>
    </section>
    }
}
