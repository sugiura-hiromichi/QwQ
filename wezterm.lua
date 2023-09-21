--🫠
local wz = require 'wezterm'
local act = wz.action

local function theme_selector(app)
	local handle = assert(io.open('/tmp/wz_nvim.txt', 'w+'), 'could not opened wz_nvim.txt')

	local rslt
	if app:find 'Dark' then
		handle:write 'dark'
		rslt = 'Nova (base16)'
	else
		handle:write 'light'
		rslt = 'Alabaster'
	end
	handle:close()
	return rslt
end

local cp_mode
if wz.gui then
	cp_mode = wz.gui.default_key_tables().copy_mode
	local meta_ops = {
		{ key = 'LeftArrow',  mods = 'NONE', action = act.AdjustPaneSize { 'Left', 1 } },
		{ key = 'RightArrow', mods = 'NONE', action = act.AdjustPaneSize { 'Right', 1 } },
		{ key = 'UpArrow',    mods = 'NONE', action = act.AdjustPaneSize { 'Up', 1 } },
		{ key = 'DownArrow',  mods = 'NONE', action = act.AdjustPaneSize { 'Down', 1 } },
	}

	for i = 1, 4 do
		table.insert(cp_mode, meta_ops[i])
	end
end

wz.on('opacity', function(window, _)
	local overrides = window:get_config_overrides() or {}
	local hndl = assert(io.open('/tmp/wz_nvim.txt', 'w+'), 'could not opened wz_nvim.txt')

	if not overrides.window_background_opacity then
		overrides.window_background_opacity = 0.45
		overrides.text_background_opacity = 0.4
		if wz.gui.get_appearance():find 'Light' then
			hndl:write 'dark'
			overrides.color_scheme = 'Nova (base16)'
		end
	else
		overrides.window_background_opacity = nil
		overrides.text_background_opacity = nil
		if wz.gui.get_appearance():find 'Light' then
			hndl:write 'light'
			overrides.color_scheme = 'Alabaster'
		end
	end
	hndl:close()
	window:set_config_overrides(overrides)
end)

return {
	show_update_window = true,
	font_size = 12.5,
	freetype_load_target = 'HorizontalLcd',
	line_height = 0.9,
	disable_default_key_bindings = true,
	keys = {
		{ key = 'c',        mods = 'SHIFT|CMD', action = act.ActivateCopyMode },
		{ key = 'F4',       mods = 'NONE',      action = act.ActivatePaneDirection 'Prev' },
		{ key = 'F5',       mods = 'NONE',      action = act.ActivatePaneDirection 'Next' },
		{ key = 'Tab',      mods = 'CTRL',      action = act.ActivateTabRelative(1) },
		{ key = 'c',        mods = 'CMD',       action = act.CopyTo 'Clipboard' },
		{ key = 'w',        mods = 'CMD',       action = act.CloseCurrentPane { confirm = false } },
		{ key = 'b',        mods = 'CMD',       action = act.EmitEvent 'bg' },
		{ key = 'o',        mods = 'CMD',       action = act.EmitEvent 'opacity' },
		{ key = 'v',        mods = 'CMD',       action = act.PasteFrom 'Clipboard' },
		{ key = 'q',        mods = 'CMD',       action = act.QuitApplication },
		{ key = 'r',        mods = 'CMD',       action = act.ReloadConfiguration },
		{ key = 'PageUp',   mods = 'SHIFT',     action = act.ScrollByPage(-1) },
		{ key = 'PageDown', mods = 'SHIFT',     action = act.ScrollByPage(1) },
		{ key = 'f',        mods = 'CMD',       action = act.Search { CaseSensitiveString = '' } },
		{ key = 't',        mods = 'CMD',       action = act.SpawnTab 'DefaultDomain' },
		{ key = 'n',        mods = 'CMD',       action = act.SpawnWindow },
		{
			key = 'd',
			mods = 'CMD|SHIFT',
			action = act.SplitHorizontal {},
		},
		{
			key = 'd',
			mods = 'CMD',
			action = act.SplitVertical {},
		},
		{ key = 'f', mods = 'CTRL|CMD', action = act.ToggleFullScreen },
	},
	key_tables = { copy_mode = cp_mode },
	color_scheme = theme_selector(wz.gui.get_appearance()),
	hide_tab_bar_if_only_one_tab = true,
	native_macos_fullscreen_mode = false,
	tab_bar_at_bottom = true,
	window_padding = { left = 0, right = 0, top = 0, bottom = 0 },
	window_decorations = 'RESIZE',
	window_close_confirmation = 'NeverPrompt',
	--mocos_window_background_blur=5,
	--window_background_image='/path/to/img.jpg' png, gif also vaild extensiton
}
