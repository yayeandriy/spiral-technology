use leptos::prelude::*;


#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main class="w-full h-screen flex bg-black items-start justify-start p-8 text-[20px]" style="line-height: 1.5;">
            <div class="flex flex-col gap-2 w-[400px]">
                <img class="w-24 h-24" src="/public/logo-white@2x.svg" alt="logo" />
                <div class="text-white text-4xl font-bold">
                    <img class="w-36" src="/public/title-white.svg" alt="logo" />
                </div>
            </div>
            <div>
                <div class="flex flex-col gap-y-8 w-[650px]">
                    
                <p>"Over the past few years, we at Spiral Technology have explored various ways to harness the potential of Augmented Reality (AR), especially focusing on mobile platforms. The specific cases we've tested could constitute their own detailed stories, but currently, our emphasis is on Non-Destructive Testing (NDT). While each industry has its unique language and processes, ultimately, they're all aiming for the same goal: ensuring quality through detailed inspection."</p>

                <p>Our app, <a class="ml-1 hover:text-indigo-500 text-indigo-300" href="https://apps.apple.com/us/app/spotternext/id6744022861" > SpotterNext</a>, leverages Augmented Reality on mobile devices to overlay critical inspection information during industrial quality checks. When working with large or intricate composite parts, spatial marking on a mobile interface offers several distinct advantages:</p>

                    <ul>
                        <li class="list-disc">Automatic defect measurements</li>
                        <li class="list-disc">Visual annotations directly on components</li>
                        <li class="list-disc">Real-time updates on defect status</li>
                        <li class="list-disc">Location-specific instructions and detailed guidelines for inspection and repair stages</li>
                        <li class="list-disc">Virtual transparency features enabling inspection of multilayer components and facilitating precise repair planning</li>
                        <li class="list-disc">Seamless collaboration and handovers among shifts, inspection teams, and repair crews</li>
                    </ul>
                </div>
                <div class="pt-8">
                    <contact class="text-fuchsia-400 " >k@spiral.technology</contact>
                </div>
                <div>
                    <a href="https://privacy.spector.vision/" class="text-indigo-600" >Privacy Policy</a>
                </div>
                <div class="pt-6 text-gray-500 text-sm">
                    <p>"©" 2025 Spiral Technology. All rights reserved.</p>
                </div>
            </div>
        </main>
    }
}
