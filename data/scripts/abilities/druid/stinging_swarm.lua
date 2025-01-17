function on_activate(parent, ability)
  local targets = parent:targets()
  
  local targeter = parent:create_targeter(ability)
  targeter:set_selection_radius(12.0)
  targeter:set_free_select(12.0)
  targeter:set_shape_circle(5.0)
  targeter:add_all_effectable(targets)
  targeter:activate()
end

function on_target_select(parent, ability, targets)
  local points = targets:affected_points()
  local surface = parent:create_surface(ability:name(), points, ability:duration())
  
  local stats = parent:stats()
  local bonus = 10 + stats.caster_level / 2 + stats.wisdom_bonus / 4
  surface:add_num_bonus("defense", -bonus)
  surface:add_num_bonus("reflex", -bonus)
  surface:add_num_bonus("melee_accuracy", -bonus)
  surface:add_num_bonus("ranged_accuracy", -bonus)
  surface:add_num_bonus("spell_accuracy", -bonus)
  
  surface:set_squares_to_fire_on_moved(3)
  local cb = ability:create_callback(parent)
  cb:set_on_surface_round_elapsed_fn("on_round_elapsed")
  cb:set_on_moved_in_surface_fn("on_moved")
  surface:add_callback(cb)
  
  local s_anim = parent:create_particle_generator("particles/circle4")
  s_anim:set_position(s_anim:param(0.0), s_anim:param(0.0))
  s_anim:set_color(s_anim:param(0.0), s_anim:param(0.0), s_anim:param(0.0), s_anim:param(1.0))
  s_anim:set_gen_rate(s_anim:param(20.0))
  s_anim:set_particle_size_dist(s_anim:fixed_dist(0.1), s_anim:fixed_dist(0.1))
  s_anim:set_particle_duration_dist(s_anim:fixed_dist(1.0))
  s_anim:set_particle_position_dist(s_anim:dist_param(s_anim:uniform_dist(-1.0, 1.0), s_anim:uniform_dist(-1.0, 1.0)),
                                    s_anim:dist_param(s_anim:uniform_dist(-1.0, 1.0), s_anim:uniform_dist(-1.0, 1.0)))
  s_anim:set_draw_above_entities()
  surface:add_anim(s_anim)
  surface:apply()
  
  ability:activate(parent)
end

function on_moved(parent, ability, targets)
  local target = targets:first()
  target:take_damage(parent, 2, 4, "Piercing", 5)
end

function on_round_elapsed(parent, ability, targets)
  local targets = targets:to_table()

  local special_index = 0
  if parent:ability_level(ability) > 1 then
    special_index = math.random(#targets)
  end

  for i = 1, #targets do
    if i == special_index then
	  game:say_line("Swarmed!", targets[i])
	  targets[i]:take_damage(parent, 15, 22, "Piercing", 8)
	else
	  targets[i]:take_damage(parent, 2, 4, "Piercing", 5)
	end
  end
  
  
end
