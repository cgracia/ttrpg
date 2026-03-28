/// Advances world time and fires tick events.
use bevy::prelude::*;

use crate::resources::{EventLog, TickEvent, WorldTime};

pub fn advance_time(
    mut time: ResMut<WorldTime>,
    mut commands: Commands,
    real_time: Res<Time>,
    log: Res<EventLog>,
) {
    if time.paused {
        return;
    }

    time.real_timer += real_time.delta_secs();
    if time.real_timer >= time.seconds_per_turn {
        time.real_timer -= time.seconds_per_turn;
        time.turn += 1;
        commands.insert_resource(TickEvent);
        let _ = log; // accessed for borrow ordering
    }
}

pub fn cleanup_tick(mut commands: Commands) {
    commands.remove_resource::<TickEvent>();
}
