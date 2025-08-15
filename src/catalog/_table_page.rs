use leptos::{html::Div, logging, prelude::*};
use leptos_use::{use_element_size, UseElementSizeReturn};

use crate::projects::{model::{Project, ProjectComparisonArea}, projects_context::use_project};


#[component]
pub fn TablePage() -> impl IntoView {
    let project_context = use_project();
    let projects = move || project_context.projects.0.get();


    let area_items = |area: ProjectComparisonArea|
    area.get_items().into_iter().map(|item| {
        view! {
            <div class="flex items-center justify-center text-center w-full">
                <div class="w-full text-center">{format!("{}", item)}</div>                
            </div>
        }
    }).collect::<Vec<_>>();

    let el = NodeRef::<Div>::new();
    let UseElementSizeReturn { width, height } = use_element_size(el);
    let stop_points = |area: ProjectComparisonArea, project: &Project, _width_value: f64| {
        let project_items = area.get_project_area_items(project);
        let project_items_strings: Vec<String> = project_items.iter().map(|item| format!("{}", item)).collect();
      
        let area_items = area.get_items();
        area_items.iter().enumerate()
        .filter(|(_, item)| {
            let item_str = format!("{}", item);
            project_items_strings.contains(&item_str)
        })
        .map(|(index, _)|  index)
        .collect::<Vec<_>>()
    };
    // let stop_points = |area: ProjectComparisonArea, project: &Project, width_value: f64| {
    //     let project_items = area.get_project_area_items(project);
    //     logging::log!("project_items for project {}: {:?}", project.id, project_items);
    //     let project_items_debug: Vec<String> = project_items.iter().map(|item| format!("{:?}", item)).collect();
    //     // let width = 1000.0;
    //     let total_area_item = area.get_items().len() as f64;
    //     let dist = 50.0 + width_value / total_area_item.max(1.0);
    //     logging::log!("Total area items: {}, Dist: {}", total_area_item, dist);
    //     let area_items = area.get_items();
    //     logging::log!("Area items for project {}: {:?}", project.id, area_items);
      

    //     area_items.iter().enumerate()
    //     .filter(|(_, item)| project_items_debug.contains(&format!("{:?}", item)))
    //     .map(|(index, _)| 50.0 + index as f64 * dist).collect::<Vec<_>>()
    // };

 
    let rullers = move |area: ProjectComparisonArea, project: &Project| {
      let area_clone = area.clone();
       let stop_points = stop_points(area, project, width.get());
        let area_items = area_clone.get_items();
        let w = (5000.0 / area_items.len() as f64) as i32;
       let points =area_items.iter()
       .enumerate()
       .map(|(i, stop)| {
        // let opacity = if stop_points.contains(&i) { "opacity-100" } else { "opacity-0" };
        let ml = if stop_points.contains(&i) { "" } else { "-ml-96" };
        let delay = 0.05 * project.id as f32;
        view! {
            <div class=format!("flex overflow-x-hidden w-full justify-center items-center flex-row-reverse transition-all h-[30px] ease-out duration-[2s] ")
             style="background: linear-gradient(
                                  to bottom,
                                  transparent 49%,   
                                  #dfdfdf 49%,         
                                  #dfdfdf 51%,         
                                  transparent 51%    
                                )"
            >
                <div class=format!("h-2 w-2 relative  rounded-full bg-black {ml} transition-all ease-out duration-[1s]")
                style=format!("transition-delay: {delay}s;") />
            </div>
        } 
       })
       .collect::<Vec<_>>();
      return view! {
          <div class="flex justify-between w-full items-center">
              {points}
          </div>
      };
    };
    // let rullers = move |area: ProjectComparisonArea, project: &Project| {
    //    let stop_points = stop_points(area, project, width.get());
    //    logging::log!("Stop points for project {}: {:?}", project.id, stop_points);
    //    stop_points.iter()
    //    .enumerate()
    //    .map(|(i, stop)| {
    //     let width = *stop as i64;
    //     let style = format!("width: {width}px");
    //     let mt = if i == 0 { "mt-0" } else { "mt-[-30px]" };
    //     view! {
    //         <div class=format!("relative {mt} ml-0 flex items-end flex-row-reverse transition-all h-[30px] w-[30px] ease-out duration-[2s]")
    //         style={style}
    //         >
    //             <div class="h-2 w-2 rounded-full bg-black -mb-[3px] -ml-1 transition-all ease-[cubic-bezier(0.95,0.05,0.795,0.035)] duration-[1s]" />
    //         </div>
    //     } 
    //    })
    //    .collect::<Vec<_>>()
    // };

    let current_area = signal::<ProjectComparisonArea>(ProjectComparisonArea::Technology);

    let comp_areas = move || ProjectComparisonArea::iter().map(|area| {
      let current_area_val= current_area.0.get();
      let is_area_current = current_area_val == area;
        view! {
            <div class="flex items-center cursor-pointer"
            class:text-gray-300={!is_area_current}
            on:click=move |_| {
                current_area.1.set(area.clone());
            }
            >
                <div class="w-full flex justify-center">
                  <div class="text-center w-full">
                    {area.to_string()}
                  </div>
                </div>
            </div>
        }
    }).collect::<Vec<_>>();

    
    

    view! {
        <div node_ref=el class="w-full -mt-12">
        
          <div class="flex w-full justify-between items-center mb-2">
              {
                move || comp_areas()
              }
          </div>
          <div class="flex w-full justify-between items-center ">
              {
                  move || area_items(current_area.0.get())
              }
          </div>
        <ol class="mt-3 w-full transition-all text-gray-500 hover:marker:text-zinc-800 cursor-pointer marker:text-zinc-200 marker:font-mono marker:font-normal">
                    {
                        move || projects().iter().map(|project| {
                            let project = project.clone();
                            view! {
                                <li class="pl-2 w-full mb-2 text-black">
                                  <div class="w-full " >
                                    {move || rullers(current_area.0.get(), &project)}                               
                                  </div>
                                </li>
                            }
                        }).collect::<Vec<_>>()
                }
                </ol>
         
        </div>
    }
}
