use gfx;
use time;
use multimap::MultiMap;

use std::collections::BTreeMap as TreeMap;

use render::gfx::{Renderer, construct_opengl_renderer};

use {PuckResult, FileResources, RenderTick};
use puck_core::app::{IdSeed, SimSettings};
use puck_core::event::*;
use puck_core::Tick;
use super::{RenderedApp, RenderSettings};
use std::collections::Bound::Included;


pub struct ReneredAppRunner<RA, R, C, F, D> where RA : RenderedApp,
                                                  R : gfx::Resources,
                                                  C : gfx::CommandBuffer<R>,
                                                  F : gfx::Factory<R>,
                                                  D : gfx::Device {
    tick: u64,
    entities: TreeMap<RA::Id, RA::Entity>,
    renderer: Renderer<R, C, F, D>,
    render_state: RA::RenderState,
}

pub const NANOSECONDS_IN_A_SECOND : u64 = 1_000_000_000u64;

pub fn run<RA>(file_resources:FileResources, sim_settings: SimSettings, render_settings:RenderSettings, render_state: RA::RenderState, initial_entities: TreeMap<RA::Id, RA::Entity>) -> PuckResult<()> where RA : RenderedApp {
    let mut renderer = construct_opengl_renderer(file_resources, render_settings.dimensions, render_settings.vsync, &render_settings.title)?;

    // start file watcher
    // start sound worker

    let mut id_seed : IdSeed = 0;

    let mut rs = render_state;

    let start_time = time::precise_time_ns();
    let mut last_time = start_time;
    let mut simulation_accu_ns = 0_u64;

    let mut last_entities : TreeMap<RA::Id, RA::Entity> = initial_entities.clone();
    let mut entities : TreeMap<RA::Id, RA::Entity> = initial_entities.clone();

    let mut tick = 0_u64;

    let per_tick_ns = NANOSECONDS_IN_A_SECOND / sim_settings.tick_rate;

    let mut to_route : Vec<_> = Vec::new();
    let mut render_events : Vec<_> = Vec::new();

    let mut running = true;

    while running {
        // check file watcher shit

        let (dimensions, input) = renderer.begin_frame(false, false);


        let time = time::precise_time_ns();


        let since_start_ns = time - start_time;
        let time_delta_ns = time - last_time;

        simulation_accu_ns += time_delta_ns;
        let mut sink = Sink::empty();
        RA::handle_input(&input, &dimensions, &entities, &mut sink);
        to_route.append(&mut sink.events);

        // ROUTE THE EVENTS

        while simulation_accu_ns > per_tick_ns {
//            println!("tick @ {:?}", tick);
            // simulate
            let simulate_tick = Tick {
                n: tick,
                tick_duration: (per_tick_ns as f64) / (NANOSECONDS_IN_A_SECOND as f64),
                tick_rate: sim_settings.tick_rate, // per second
            };

            last_entities = entities;

            let mut entity_events = MultiMap::new();

            for ev in to_route {
//                println!("handle event -> {:?}", ev);
                match ev  {
                    Event::Shutdown => running = false,
                    Event::SpawnEvent(id, entity) => {
                        let use_id = match RA::modify_id(&id, id_seed) {
                            Some(new_id) => {
                                id_seed += 1; // seed was consumed to generate the id, increment it
                                new_id
                            },
                            None => id,
                        };
                        last_entities.insert(use_id, entity);
                    },
                    Event::Delete(id) => {
                        last_entities.remove(&id);
                    },
                    Event::DeleteRange(from, to) => {
                        let mut to_delete = Vec::new();
                        for (k, _) in last_entities.range((Included(&from), Included(&to))) {
                            to_delete.push(k.clone());
                        }
                        for k in to_delete {
                            last_entities.remove(&k);
                        }
                    },
                    Event::EntityEvent(id, entity_event) => entity_events.insert(id, entity_event),
                    Event::RenderEvent(render_event) => render_events.push(render_event),
                }
            }

            to_route = Vec::new();

            entities = last_entities.iter().map(|(id, e)| {
                let mut entity = e.clone();

                // handle events from last frame
                if let Some(evs) = entity_events.get_vec(id) {
                    for event in evs {
                        let mut sink = Sink::empty();
                        RA::handle_entity_event(event, id, &mut entity, &mut sink);
                        to_route.append(&mut sink.events);
                    }
                }

                // simulate entity
//                println!("simulate -> {:?} {:?}", id, e);
                let mut combined_sink = CombinedSink::empty();
                RA::simulate(simulate_tick, &last_entities, id, &entity, &mut combined_sink);


                to_route.append(&mut combined_sink.routed.events);

                // handle self effects immediately
                for event in &combined_sink.mine.events {
                    let mut sink = Sink::empty();
                    RA::handle_entity_event(event, id, &mut entity, &mut sink);
                    to_route.append(&mut sink.events);
                }

                (id.clone(), entity)
            }).collect();


            tick += 1;
            simulation_accu_ns -= per_tick_ns;
        }

        // render
        let render_tick = RenderTick {
            n: tick,
            accu_alpha: (simulation_accu_ns as f64) / (per_tick_ns as f64), // percentage of a frame that has accumulated
            tick_rate: sim_settings.tick_rate, // per second
        };

        for render_event in render_events {
            RA::handle_render_event(&render_event, &mut rs);
        }
        render_events = Vec::new();
//        println!("render");
        let ok = renderer.load_resources(false, false);
        if !ok {
            println!("renderer is not ok");
        }
        RA::render(render_tick, &dimensions, &entities, &mut rs, &mut renderer);

        if input.close {
            running = false;
        }

        last_time = time;
    }



    Ok(())
}