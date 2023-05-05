#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::model::Computer;

mod model;
mod parse;
mod ui;
mod utils;

use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (computer, set_computer) = create_signal(cx, Computer::new());

    let nums = move || computer().general_memory.data.iter().map(|e| view!{cx, <li>{e.get()}</li>}).collect::<Vec<_>>();

    let step10000 = move |_| set_computer.update(|comp| {comp.nth(10000);});
    let step100 = move |_| set_computer.update(|comp| {comp.nth(100);});
    let step1 = move |_| set_computer.update(|comp| {comp.nth(1);});

    view! {
        cx,
        "My cool computer" <br/> "General memory overview:" <br/>
        <button on:click={step10000}>"Step 10000"</button>
        <button on:click={step100}>"Step 100"</button>
        <button on:click={step1}>"Step 1"</button>
        <ul>
        {
            nums
        }
        </ul>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

// const MANY_COUNTERS: usize = 1000;

// type CounterHolder = Vec<(usize, (ReadSignal<i32>, WriteSignal<i32>))>;

// #[derive(Copy, Clone)]
// struct CounterUpdater {
//     set_counters: WriteSignal<CounterHolder>,
// }

// #[component]
// pub fn Counters(cx: Scope) -> impl IntoView {
//     let (next_counter_id, set_next_counter_id) = create_signal(cx, 0);
//     let (counters, set_counters) = create_signal::<CounterHolder>(cx, vec![]);
//     provide_context(cx, CounterUpdater { set_counters });

//     let add_counter = move |_| {
//         let id = next_counter_id.get();
//         let sig = create_signal(cx, 0);
//         set_counters.update(move |counters| counters.push((id, sig)));
//         set_next_counter_id.update(|id| *id += 1);
//     };

//     let add_many_counters = move |_| {
//         let next_id = next_counter_id.get();
//         let new_counters = (next_id..next_id + MANY_COUNTERS).map(|id| {
//             let signal = create_signal(cx, 0);
//             (id, signal)
//         });

//         set_counters.update(move |counters| counters.extend(new_counters));
//         set_next_counter_id.update(|id| *id += MANY_COUNTERS);
//     };

//     let clear_counters = move |_| {
//         set_counters.update(|counters| counters.clear());
//     };

//     view! { cx,
//         <div>
//             <button on:click=add_counter>
//                 "Add Counter"
//             </button>
//             <button on:click=add_many_counters>
//                 {format!("Add {MANY_COUNTERS} Counters")}
//             </button>
//             <button on:click=clear_counters>
//                 "Clear Counters"
//             </button>
//             <p>
//                 "Total: "
//                 <span>{move ||
//                     counters.get()
//                         .iter()
//                         .map(|(_, (count, _))| count.get())
//                         .sum::<i32>()
//                         .to_string()
//                 }</span>
//                 " from "
//                 <span>{move || counters.with(|counters| counters.len()).to_string()}</span>
//                 " counters."
//             </p>
//             <ul>
//                 <For
//                     each={move || counters.get()}
//                     key={|counter| counter.0}
//                     view=move |cx, (id, (value, set_value))| {
//                         view! {
//                             cx,
//                             <Counter id value set_value/>
//                         }
//                     }
//                 />
//             </ul>
//         </div>
//     }
// }

// #[component]
// fn Counter(
//     cx: Scope,
//     id: usize,
//     value: ReadSignal<i32>,
//     set_value: WriteSignal<i32>,
// ) -> impl IntoView {
//     let CounterUpdater { set_counters } = use_context(cx).unwrap();

//     let input = move |ev| set_value.set(event_target_value(&ev).parse::<i32>().unwrap_or_default());

//     view! { cx,
//         <li>
//             <button on:click=move |_| set_value.update(move |value| *value -= 1)>"-1"</button>
//             <input type="text"
//                 prop:value={move || value.get().to_string()}
//                 on:input=input
//             />
//             <span>{move || value.get().to_string()}</span>
//             <button on:click=move |_| set_value.update(move |value| *value += 1)>"+1"</button>
//             <button on:click=move |_| set_counters.update(move |counters| counters.retain(|(counter_id, _)| counter_id != &id))>"x"</button>
//         </li>
//     }
// }
