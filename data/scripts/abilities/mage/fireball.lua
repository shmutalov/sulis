fireball_radius = 5.0

function on_activate(parent, ability)
  local targets = parent:targets()
  
  local targeter = parent:create_targeter(ability)
  targeter:set_free_select(15.0)
  targeter:set_selection_radius(15.0)
  targeter:set_shape_object_size("9by9round")
  targeter:invis_blocks_affected_points(true)
  targeter:add_all_effectable(targets)
  targeter:activate()
end

function on_target_select(parent, ability, targets)
  local selected_point = targets:selected_point()
  local speed = 10.0
  local dist = parent:dist_to_point(selected_point)
  local duration = dist / speed
  local vx = (selected_point.x - parent:center_x()) / duration
  local vy = (selected_point.y - parent:center_y()) / duration
  
  local cb = ability:create_callback(parent)
  cb:add_targets(targets)
  cb:set_on_anim_update_fn("create_explosion")
  
  local gen = parent:create_particle_generator("fire_particle", duration)
  gen:set_position(gen:param(parent:center_x(), vx), gen:param(parent:center_y(), vy))
  gen:set_gen_rate(gen:param(70.0))
  gen:set_initial_gen(35.0)
  gen:set_particle_size_dist(gen:fixed_dist(0.7), gen:fixed_dist(0.7))
  gen:set_particle_position_dist(gen:dist_param(gen:uniform_dist(-0.3, 0.3), gen:uniform_dist(-vx / 5.0, 0.0)),
    gen:dist_param(gen:uniform_dist(-0.3, 0.3), gen:uniform_dist(-vy / 5.0, 0.0)))
  gen:set_particle_duration_dist(gen:fixed_dist(0.6))
  gen:add_callback(cb, duration - 0.1)
  gen:activate()
  
  ability:activate(parent)
end

function create_explosion(parent, ability, targets)
  local anim = parent:wait_anim(0.3)
  local cb = ability:create_callback(parent)
  cb:add_targets(targets)
  cb:set_on_anim_complete_fn("create_fire_surface")
  anim:set_completion_callback(cb)
  anim:activate()
  
  local duration = 1.2
  
  local position = targets:selected_point()
  
  local gen = parent:create_particle_generator("fire_particle", duration)
  gen:set_initial_gen(500.0)
  gen:set_gen_rate(gen:param(100.0, 0, -500, -500))
  gen:set_position(gen:param(position.x), gen:param(position.y))
  gen:set_particle_size_dist(gen:fixed_dist(0.7), gen:fixed_dist(0.7))
  local speed = 1.5 * fireball_radius / 0.6
  gen:set_particle_position_dist(gen:dist_param(gen:uniform_dist(-0.1, 0.1), gen:angular_dist(0.0, 2 * math.pi, 0, speed)))
  gen:set_particle_duration_dist(gen:fixed_dist(0.6))
  
  local targets = targets:to_table()
  for i = 1, #targets do
    local cb = ability:create_callback(parent)
	cb:add_target(targets[i])
	cb:set_on_anim_update_fn("attack_target")
    gen:add_callback(cb, targets[i]:dist_to_point(position) / speed * 1.5)
  end
  
  gen:activate()
end

function attack_target(parent, ability, targets)
  local target = targets:first()
  
  if target:is_valid() then
    local stats = parent:stats()
	local min_dmg = 15 + stats.caster_level / 3 + stats.intellect_bonus / 6
    local max_dmg = 25 + stats.intellect_bonus / 3 + stats.caster_level * 0.667
    parent:special_attack(target, "Reflex", "Spell", min_dmg, max_dmg, 0, "Fire")
  end
end

function create_fire_surface(parent, ability, targets)
  local points = targets:random_affected_points(0.7)
  local surf = parent:create_surface("Fire", points, 2)
  surf:set_squares_to_fire_on_moved(3)
  
  local cb = ability:create_callback(parent)
  cb:set_on_surface_round_elapsed_fn("on_round_elapsed")
  cb:set_on_moved_in_surface_fn("on_moved")
  surf:add_callback(cb)
  
  local gen = parent:create_particle_generator("fire_particle")
  gen:set_alpha(gen:param(0.75))
  gen:set_gen_rate(gen:param(30.0))
  gen:set_position(gen:param(0.0), gen:param(0.0))
  gen:set_particle_size_dist(gen:fixed_dist(0.5), gen:fixed_dist(0.5))
  gen:set_particle_duration_dist(gen:fixed_dist(0.6))
  gen:set_particle_position_dist(gen:dist_param(gen:uniform_dist(-0.5, 0.5), gen:uniform_dist(-0.1, 0.1)),
								 gen:dist_param(gen:uniform_dist(0.0, 0.5), gen:uniform_dist(-2.0, -3.0)))
  gen:set_draw_above_entities()
  surf:add_anim(gen)
  
  local below = parent:create_anim("particles/circle16")
  below:set_draw_below_entities()
  below:set_position(below:param(-0.25), below:param(-0.25))
  below:set_particle_size_dist(below:fixed_dist(1.5), below:fixed_dist(1.5))
  below:set_color(below:param(0.8), below:param(0.5), below:param(0.0), below:param(0.2))
  surf:add_anim(below)
  
  surf:apply()
end

function on_moved(parent, ability, targets)
  local target = targets:first()
  target:take_damage(parent, 3, 6, "Fire", 2)
end

function on_round_elapsed(parent, ability, targets)
  local targets = targets:to_table()
  for i = 1, #targets do
	targets[i]:take_damage(parent, 3, 6, "Fire", 2)
  end
end
