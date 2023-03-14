use glfw::{Glfw, Context, Key, Action};
use vplugin::PluginManager;

fn main() {
        /* VPlugin requires a logger to be active in order to report any errors through the console. */
        env_logger::init();

        /* Here we just initialize our windowing library and create an OpenGL window. Nothing of interest. */
        let mut glfw = glfw::init::<()>(None).expect("Couldn't initialize GLFW");
        let (mut window, recv) = glfw
            .create_window(1024,
                           768,
                           "VPlugin Example",
                           glfw::WindowMode::Windowed)
                           .expect("Couldn't create window");
        window.make_current();
        window.set_key_polling(true);

        /*
         * Now the fun part begins: We first have to create the plugin manager
         * and configure it properly. In the second line, we set the entry point to
         * the custom entry point that we specified in the documentation.
         */
        let mut plug_mgr = PluginManager::new();
        plug_mgr.set_entry_point("vplugin_example_init");

        /* 
         * Since we can load multiple plugins, we will store them all in this Vector.
         * We will need to keep ownership of this Vector forever since it owns all
         * the plugins we load.
         */
        let mut plugins = Vec::<vplugin::Plugin>::new();

        /*
         * We will also need to keep the hooks we want to run somewhere.
         * Realistically, you would put the actual [vplugin::Plugin](vplugin::Plugin) struct
         * along with all the supported hooks in a single struct so you only have to keep
         * track of one vector. In this case however, we will seperate them to make it easier
         * to understand.
         */
        let mut redraw_hooks = Vec::<Option<unsafe extern "C" fn(())>>::new();

        /*
         * Each command line argument is considered a plugin filename so we will load them all
         * at once using an iterator. We need to skip the very first argument because it's the
         * actual program name.
         */
        std::env::args().skip(1).for_each(|arg|{
                let plugin = plug_mgr.load_plugin(&arg).expect("Couldn't load plugin");
                plugins.push(plugin);
        });

        /*
         * Stores the available plugin.
         */
        let mut plugin_index = 0;

        /*
         * Last, we will get all the hooks before running the actual program loop.
         * This would be the point where you would also check if a plugin is the correct one
         * for your program and if any required symbols are missing.
         */
        for plugin in plugins.iter_mut() {
            plug_mgr.begin_plugin(plugin).expect("Couldn't start plugin.");

            let redraw_hook = plug_mgr.get_custom_hook::<(), ()>(&plugin, "vplugin_example_on_redraw");
            if redraw_hook.is_err() {
                redraw_hooks.push(None);
            } else {
                redraw_hooks.push(Some(redraw_hook.unwrap()));
            }
        }

        while !window.should_close() {
                /* 
                 * Now, we actually take the functions of the plugin intended for this specific stage
                 * of our application. If you had more hooks to query for, then you would do that
                 * in a seperate loop.
                 */
                for on_redraw_hook in redraw_hooks.iter() {
                    if on_redraw_hook.is_some() {
                        unsafe { on_redraw_hook.unwrap()(()); }
                    }
                }
                window.swap_buffers();
                glfw.poll_events();

                for (_, event) in glfw::flush_messages(&recv) {
                    match event {
                        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                            window.set_should_close(true)
                        },
                        glfw::WindowEvent::Key(Key::Enter, _, Action::Release, _) => {
                            if plugins.get(plugin_index).is_none() {
                                break;
                            }
                            plugins.get_mut(plugin_index).unwrap().terminate().expect("Couldn't terminate");
                            plugin_index += 1;
                        },
                        _ => {},
                    }
                }
        }
}
