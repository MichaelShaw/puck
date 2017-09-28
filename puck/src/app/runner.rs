use gfx;
use time;
use std::collections::BTreeMap as TreeMap;

use render::gfx::{Renderer, construct_opengl_renderer};

use {PuckResult, FileResources, RenderTick};
use puck_core::app::SimSettings;
use puck_core::Tick;
use super::{RenderedApp, RenderSettings};



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

pub fn run<RA>(app: RA, file_resources:FileResources, sim_settings: SimSettings, render_settings:RenderSettings, render_state: RA::RenderState) -> PuckResult<()> where RA : RenderedApp {
    let mut renderer = construct_opengl_renderer(file_resources, render_settings.dimensions, render_settings.vsync, &render_settings.title)?;

    // start file watcher
    // start sound worker


    let mut rs = render_state;

    let start_time = time::precise_time_ns();
    let mut last_time = start_time;
    let mut simulation_accu_ns = 0_u64;

    let mut last_entities : TreeMap<RA::Id, RA::Entity> = TreeMap::new();
    let mut entities : TreeMap<RA::Id, RA::Entity> = TreeMap::new();

    let mut tick = 0_u64;

    let per_tick_ns = NANOSECONDS_IN_A_SECOND / sim_settings.tick_rate;

    'main: loop {
        // check file watcher shit

        let (dimensions, input) = renderer.begin_frame(false, false);
        let time = time::precise_time_ns();
        let since_start_ns = time - start_time;
        let time_delta_ns = time - last_time;

        simulation_accu_ns += time_delta_ns;

        let events = RA::handle_input(&input, &dimensions, &entities);

        // ROUTE THE EVENTS

        while simulation_accu_ns > per_tick_ns {
            // simulate
            let simulate_tick = Tick {
                n: tick,
                tick_duration: (per_tick_ns as f64) / (NANOSECONDS_IN_A_SECOND as f64),
                tick_rate: sim_settings.tick_rate, // per second
            };

            for (id, entity) in entities.iter() {
                let (self_events, route_events) = RA::simulate(simulate_tick, &entities, id, entity);

            }

            tick += 1;
            simulation_accu_ns -= per_tick_ns;
        }

        // render
        let render_tick = RenderTick {
            n: tick,
            accu_alpha: (simulation_accu_ns as f64) / (per_tick_ns as f64), // percentage of a frame that has accumulated
            tick_rate: sim_settings.tick_rate, // per second
        };
        RA::render(render_tick, &entities, &mut rs, &mut renderer);
    }



    Ok(())
}