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

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use sulis_core::io::event;
use sulis_core::ui::{Callback, Widget, WidgetKind};
use sulis_core::widgets::{Button, Label, ProgressBar};
use sulis_state::{ChangeListener, EntityState, GameState};

use crate::CharacterBuilder;

pub const NAME: &str = "portrait_view";

pub struct PortraitView {
    entity: Rc<RefCell<EntityState>>,
}

impl PortraitView {
    pub fn new(entity: Rc<RefCell<EntityState>>) -> Rc<RefCell<PortraitView>> {
        Rc::new(RefCell::new(PortraitView { entity }))
    }
}

impl WidgetKind for PortraitView {
    widget_kind!(NAME);

    fn on_add(&mut self, widget: &Rc<RefCell<Widget>>) -> Vec<Rc<RefCell<Widget>>> {
        let mut entity = self.entity.borrow_mut();
        entity
            .actor
            .listeners
            .add(ChangeListener::invalidate(NAME, widget));

        let portrait = Widget::with_theme(Label::empty(), "portrait");
        if let Some(ref image) = entity.actor.actor.portrait {
            portrait
                .borrow_mut()
                .state
                .add_text_arg("image", &image.id());
        }

        let frac = entity.actor.hp() as f32 / entity.actor.stats.max_hp as f32;
        let hp_bar = Widget::with_theme(ProgressBar::new(frac), "hp_bar");
        hp_bar
            .borrow_mut()
            .state
            .add_text_arg("cur_hp", &entity.actor.hp().to_string());
        hp_bar
            .borrow_mut()
            .state
            .add_text_arg("max_hp", &entity.actor.stats.max_hp.to_string());

        let class_stat_bar = match entity.actor.actor.base_class().displayed_class_stat() {
            None => {
                let widget = Widget::empty("class_stat_bar");
                widget.borrow_mut().state.set_visible(false);
                widget
            },
            Some(ref stat) => {
                let cur = entity.actor.current_class_stat(&stat.id);
                let max = entity.actor.stats.class_stat_max(&stat.id);
                let frac = cur.divide(&max);
                let bar = Widget::with_theme(ProgressBar::new(frac), "class_stat_bar");

                {
                    let state = &mut bar.borrow_mut().state;
                    state.add_text_arg("cur_stat", &cur.to_string());
                    state.add_text_arg("max_stat", &max.to_string());
                    state.add_text_arg("stat_name", &stat.name);
                }
                bar
            }
        };

        let entity_ref = Rc::clone(&self.entity);
        let level_up = Widget::with_theme(Button::empty(), "level_up");
        level_up
            .borrow_mut()
            .state
            .add_callback(Callback::new(Rc::new(move |widget, _| {
                let root = Widget::get_root(&widget);
                let window =
                    Widget::with_defaults(CharacterBuilder::level_up(Rc::clone(&entity_ref)));
                window.borrow_mut().state.set_modal(true);
                Widget::add_child_to(&root, window);
            })));
        level_up
            .borrow_mut()
            .state
            .set_visible(entity.actor.has_level_up());
        level_up
            .borrow_mut()
            .state
            .set_enabled(!GameState::is_combat_active());

        widget
            .borrow_mut()
            .state
            .set_enabled(!entity.actor.is_dead());

        let icons = Widget::empty("icons");
        let mgr = GameState::turn_manager();
        let mgr = mgr.borrow();
        for index in entity.actor.effects_iter() {
            let effect = match mgr.effect_checked(*index) {
                None => continue,
                Some(effect) => effect,
            };

            let icon = match effect.icon() {
                None => continue,
                Some(icon) => icon,
            };

            let icon_widget = Widget::with_theme(Label::empty(), "icon");
            icon_widget
                .borrow_mut()
                .state
                .add_text_arg("icon", &icon.icon);
            icon_widget
                .borrow_mut()
                .state
                .add_text_arg("text", &icon.text);
            Widget::add_child_to(&icons, icon_widget);
        }

        vec![portrait, hp_bar, class_stat_bar, level_up, icons]
    }

    fn on_mouse_enter(&mut self, widget: &Rc<RefCell<Widget>>) -> bool {
        self.super_on_mouse_enter(widget);

        let area_state = GameState::area_state();
        let targeter = area_state.borrow_mut().targeter();

        if let Some(targeter) = targeter {
            let x = self.entity.borrow().location.x;
            let y = self.entity.borrow().location.y;

            let mut targeter = targeter.borrow_mut();
            targeter.on_mouse_move(x, y);
        }
        true
    }

    fn on_mouse_release(&mut self, widget: &Rc<RefCell<Widget>>, kind: event::ClickKind) -> bool {
        self.super_on_mouse_release(widget, kind);

        let area_state = GameState::area_state();
        let targeter = area_state.borrow_mut().targeter();

        if let Some(targeter) = targeter {
            let mut targeter = targeter.borrow_mut();
            targeter.on_activate();
        } else {
            GameState::set_selected_party_member(Rc::clone(&self.entity));
        }

        true
    }
}
