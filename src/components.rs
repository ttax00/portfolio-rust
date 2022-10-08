use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <header class="fixed z-10 w-screen bg-gray-500 dark:bg-gray-800">
        <nav class="mx-auto px-2 sm:px-6 lg:px-8 grid grid-cols-2 md:grid-cols-3 gap-2">
            <div class="relative flex items-center justify-start h-16 col-span-1 md:col-span-2">
                <a href="#introducton" class="text-black dark:text-white">{"Portfolio"}</a>
                <div class="form-check form-switch ml-8">
                    <input class="element-toggle" type="checkbox" role="switch"  />
                </div>
                <img src="https://upload.wikimedia.org/wikipedia/commons/3/37/Darkmode_moon_shining_stars.svg"
                    class="h-10 w-10 ml-3 grayscale" alt="dark mode" />
            </div>
            <div class="relative hidden sm:flex items-center justify-end h-16 text-black dark:text-white">
                <div class="mr-3">{"Made With:"}</div>
                <a href="https://www.typescriptlang.org/">
                    <img src="https://upload.wikimedia.org/wikipedia/commons/4/4c/Typescript_logo_2020.svg"
                    class="h-10 w-10 mr-3" alt="Typescript" />
                </a>
                <a href="https://reactjs.org/" >
                    <img src="https://upload.wikimedia.org/wikipedia/commons/a/a7/React-icon.svg"
                    class="object-fit h-10 w-10 mr-3" alt="React" />
                </a>
                <a href="https://tailwindcss.com/">
                    <img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Tailwind_CSS_Logo.svg"
                    class="h-10 w-10" alt="Tailwind CSS" />
                </a>
            </div>
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
                    {"Welcome, I'm Tech."}
                </span>
                <br />
                <span class="motion-safe:animation delay-1s fade-in md:text-4xl">
                    {"A multi-lingual, multi-disciplinary, self-taught hobbyist developer."}
                </span>
            </p>
            <p class="motion-safe:animation delay-2s fade-in text-left mt-4 text-sm md:text-xl">
                {"I primary work on curious bite-sized projects, with more emphasis on discovery and understanding."}
                {"Currently making an effort to learn "} <a href="https://www.rust-lang.org/" class="text-blue-600 font-bold dark:text-red-700">{"Rust"}</a>{"."}
            </p>
            <div class="motion-safe:animation delay-3s fade-in self-start mt-8">
                <a class="btn-primary block motion-safe:animation delay-4s animate-bounce" href="https://github.com/TechTheAwesome">
                    {"Checkout my projects on github!"}
                </a>
            </div>
            <div class="self-center motion-safe:animation delay-3s fade-in">
                <a href="#technologies">
                    <img src="https://upload.wikimedia.org/wikipedia/commons/9/9d/Arrow-down.svg"
                        class="h-24 w-24" alt="scroll down" />
                </a>
            </div>
        </div>
    </section>
    }
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
                    <img src="https://upload.wikimedia.org/wikipedia/commons/a/a7/React-icon.svg" class="object-fit h-24 w-24" alt="React" />
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
                    class="h-24 w-24" alt="Rust" />
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
