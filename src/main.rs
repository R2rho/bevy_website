use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};


const FRAME_RATE: f64 = 60.0;
fn main() {
    App::new()
        .add_plugins(MinimalPlugins
                .set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1./FRAME_RATE)))
        )
        .add_systems(Startup, start_timer)
        .add_systems(Update, second_counter)
        .run();
}

#[derive(Resource)]
struct SecondTimer{
    timer: Timer,
}

fn  start_timer(mut commands: Commands) {
    //Start the one second timer
    commands.insert_resource(SecondTimer{
        // create the repeating timer
        // timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    })
}

fn second_counter(time: Res<Time>, mut second_timer: ResMut<SecondTimer>){
    if second_timer.timer.finished() {
        println!("Seconds elapsed: {:?}", second_timer.timer.duration())
    }
    second_timer.timer.tick(Duration::from_secs_f64(time.delta_seconds_f64()));
}