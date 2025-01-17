max_dist = 14

function on_activate(parent, ability)
  local targets = parent:targets():without_self()
  
  targeter = parent:create_targeter(ability)
  targeter:set_selection_radius(max_dist)
  targeter:set_free_select(max_dist * 2)
  targeter:set_shape_cone(parent:center_x(), parent:center_y(), 1.0, max_dist, math.pi / 4) 
  targeter:add_all_effectable(targets)
  targeter:invis_blocks_affected_points(true)
  targeter:activate()
end

function on_target_select(parent, ability, targets)
  local pos = targets:selected_point()
  
  local delta_x = pos.x - parent:center_x()
  local delta_y = pos.y - parent:center_y()
  local angle = game:atan2(delta_x, delta_y)
  
  local duration = 1.5
  
  gen = parent:create_particle_generator("wind_particle", duration)
  gen:set_position(gen:param(parent:center_x()), gen:param(parent:center_y()))
  gen:set_gen_rate(gen:param(0.0))
  gen:set_initial_gen(500.0)
  gen:set_particle_size_dist(gen:fixed_dist(0.7), gen:fixed_dist(0.7))
  gen:set_particle_position_dist(
    gen:dist_param(gen:uniform_dist(-0.1, 0.1),
    gen:angular_dist(angle - math.pi / 8, angle + math.pi / 8, 22, 30)))
    
  gen:set_particle_duration_dist(gen:fixed_dist(0.6))
  
  local targets = targets:to_table()
  for i = 1, #targets do
    local dist = parent:dist_to_entity(targets[i])
    local cb_dur = 0.5 * duration * (1 - dist / max_dist)
    -- fire callback for further targets first, so they move out of the way of the closer targets
    local cb = ability:create_callback(parent)
	cb:add_target(targets[i])
	cb:set_on_anim_update_fn("push_target")
    gen:add_callback(cb, cb_dur)
  end
  
  gen:activate()
  ability:activate(parent)
end

function push_target(parent, ability, targets)
  local target = targets:first()
  local stats = parent:stats()

  local hit = parent:special_attack(target, "Reflex", "Spell")
  
  local pushback_dist = math.floor(8 + stats.caster_level / 3 + stats.intellect_bonus / 6 - target:width())
  if hit:is_miss() then
    pushback_dist = pushback_dist - 4
  elseif hit:is_graze() then
    pushback_dist = pushback_dist - 2
  elseif hit:is_hit() then
    -- do nothing
  elseif hit:is_crit() then
    pushback_dist = pushback_dist + 2
  end
  
  if pushback_dist < 1 then
    return
  end
  
  -- compute the normalized direction to push
  local target_x = target:x()
  local target_y = target:y()
  local dir_x = target_x - parent:x()
  local dir_y = target_y - parent:y()
  local mag = math.sqrt(dir_x * dir_x + dir_y * dir_y)
  local x_norm = dir_x / mag
  local y_norm = dir_y / mag
  
  local dest_x = target_x
  local dest_y = target_y
  
  local total_dist = 0
  -- go along the direction, checking until we hit an impassable spot
  for dist = 1, pushback_dist do
    local test_x = math.floor(target_x + x_norm * dist + 0.5)
	local test_y = math.floor(target_y + y_norm * dist + 0.5)
	
	if not game:is_passable(target, test_x, test_y) then
	  break
	end
	
	dest_x = test_x
	dest_y = test_y
	total_dist = dist
  end
  
  local push_damage_base = pushback_dist - total_dist
  if push_damage_base > 0 then
    target:take_damage(parent, push_damage_base * 2 - 2, push_damage_base * 2 + 2, "Crushing")
  end
  
  -- return if the result is to not move the target
  if dest_x == target_x and dest_y == target_y then
    return
  end
  local dest = { x = dest_x, y = dest_y }
  
  -- move the target now (since we know the dest is valid now) and hide it with a subpos animation
  target:teleport_to(dest)
  
  local subpos_x = dest_x - target_x
  local subpos_y = dest_y - target_y
  target:set_subpos(-subpos_x, -subpos_y)
  
  -- create the movement animation for the computed destination
  local speed = 300 * game:anim_base_time()
  local duration = total_dist / speed
  local anim = target:create_subpos_anim(duration)
  anim:set_position(anim:param(-subpos_x, subpos_x / duration), anim:param(-subpos_y, subpos_y / duration))
  anim:activate()
end
