# Recipe

1. Extract mod to mod dir in a subdir of the archive name
2. Get relative path between mod installation dir and game install dir
3. Find the `main_init.cfg` file (this might be hard)
4. Prepend the relative path to all paths UNLESS they have the default value, this includes:
   - ConfigFiles.Resources unless "resources.cfg"
   - ConfigFiles.Materials unless "materials.cfg"
   - ConfigFiles.Game unless "config/game.cfg"
   - ConfigFiles.Menu unless "config/menu.cfg"
   - ConfigFiles.PreMenu unless "config/pre_menu.cfg"
   - ConfigFiles.Demo unless "config/demo.cfg"
   - ConfigFiles.DefaultMainSettings unless "config/default_main_settings.cfg"
   - ConfigFiles.DefaultMainSettingsLow unless "launcher/default_main_settings_low.cfg"
   - ConfigFiles.DefaultMainSettingsMedium unless "launcher/default_main_settings_medium.cfg"
   - ConfigFiles.DefaultMainSettingsHigh unless "launcher/default_main_settings_high.cfg"
   - ConfigFiles.DefaultUserSettings unless "config/default_user_settings.cfg"
   - ConfigFiles.DefaultUserKeys unless "config/default_user_keys.cfg"
   - ConfigFiles.DefaultBaseLanguage unless "config/base_english.lang"
   - Directories.BaseLanguageFolder unless "config/"
   - Directories.GameLanguageFolder unless "config/lang/"
   - Directories.CustomStoryPath unless "custom_stories"
   - StartMap.Folder unless "maps/main/"
5. Find the `resources.cfg` based on the path in `main_init.cfg`, and then add a new entry at the top for the new mod folder location. Also update all paths that aren't default to be prepended with the relative mod folder path.
6. Find `game.cfg` and prepend the path to:
   - Insanity.EventsFile unless "misc/main_sanity_events.cfg"
