//  This file is part of Sulis, a turn based RPG written in Rust.
//  Copyright 2018 Jared Stephen
//
//  Sulis is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  Sulis is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with Sulis.  If not, see <http://www.gnu.org/licenses/>

use std::cell::RefCell;
use std::rc::Rc;

use rlua;

use crate::script::script_callback;
use crate::{animation::Anim, EntityState, GameState, Script};
use sulis_core::config::Config;

pub struct AI {
    ai: Option<EntityAI>,
    next_state: State,
}

impl AI {
    pub fn new() -> AI {
        AI {
            ai: None,
            next_state: State::Run,
        }
    }

    pub fn update(&mut self, entity: Rc<RefCell<EntityState>>) {
        if GameState::is_modal_locked() {
            return;
        }

        if entity.borrow().is_party_member() {
            self.ai = None;
            return;
        }

        let assign = match self.ai {
            None => true,
            Some(ref ai) => !Rc::ptr_eq(&ai.entity, &entity),
        };

        if assign {
            debug!(
                "Initialize round AI for '{}'",
                entity.borrow().actor.actor.name
            );
            self.ai = Some(EntityAI::new(&entity));
            self.next_state = State::Wait(20);
        }

        if let Some(ref mut ai) = self.ai {
            if GameState::has_blocking_animations(&ai.entity) {
                return;
            }

            self.next_state = match self.next_state {
                State::Run => ai.run_script(),
                State::Wait(time) => {
                    ai.wait(time);
                    State::Run
                }
                State::End => {
                    debug!(
                        "AI for '{}' is ending.",
                        ai.entity.borrow().actor.actor.name
                    );
                    let turn_mgr = GameState::turn_manager();
                    let cbs = turn_mgr.borrow_mut().next();
                    script_callback::fire_round_elapsed(cbs);
                    State::Run
                }
            };
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum State {
    Run,
    Wait(u32),
    End,
}

impl rlua::UserData for State {}

const MAX_ACTIONS: u32 = 10;

struct EntityAI {
    entity: Rc<RefCell<EntityState>>,
    actions_taken_this_turn: u32,
}

impl EntityAI {
    fn new(entity: &Rc<RefCell<EntityState>>) -> EntityAI {
        EntityAI {
            entity: Rc::clone(entity),
            actions_taken_this_turn: 0,
        }
    }

    fn wait(&self, time: u32) {
        debug!(
            "AI for '{}' is waiting.",
            self.entity.borrow().actor.actor.name
        );
        let wait_time = Config::animation_base_time_millis() * time;
        let anim = Anim::new_wait(&self.entity, wait_time);
        GameState::add_animation(anim);
    }

    fn run_script(&mut self) -> State {
        if self.actions_taken_this_turn == MAX_ACTIONS {
            return State::End;
        }

        if self.entity.borrow().actor.actor.ai.is_none() {
            return State::End;
        }

        self.actions_taken_this_turn += 1;

        Script::ai(&self.entity, "ai_action")
    }
}
