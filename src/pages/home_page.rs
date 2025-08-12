use leptos::prelude::*;

use crate::posts_mock_data::ProjectDatabase;


#[component]
pub fn HomePage() -> impl IntoView {
    let db = ProjectDatabase::new();


    let is_item_selected = signal(true);
    view! {
        <main class="w-screen h-screen flex justify-start bg-white leading-[1.01] items-start text-black p-8 text-[20px] pl-[200px] pt-16" style="line-height: 1.5;">
            <div class="flex flex-col gap-2 pt-[80px] ">
                <img class="w-24 h-24" src="/public/logo-black@2x.svg" alt="logo" />               
            </div>
            <div class="flex" >
                <div class="flex flex-col gap-y-2 pl-24">
                    <ol class="list-decimal transition-all text-gray-500 hover:marker:text-zinc-800 cursor-pointer marker:text-zinc-200 marker:font-mono marker:font-normal">
                    {
                        db.get_all_projects().iter().enumerate().map(|(index, project)| {
                            let project = project.clone();
                            view! {
                                
                                    <li class="pl-2 mb-2 text-black" >
                                        // <div class="text-slate-400 w-8 text-right font-mono">{index + 1}"."</div>
                                        <div>{project.title}</div>
                                    </li>
                                
                                    // <div class="flex">
                                    //     <div class="text-slate-400 w-8 text-right font-mono">{index + 1}"."</div>
                                    //     <div>{project.title}</div>
                                    // </div>
                                    // <div class="text-sm">{project.description}</div>
                                
                            }
                        }).collect::<Vec<_>>()
                }
                </ol>
                </div> 
               </div> 
        </main>
    }
}
