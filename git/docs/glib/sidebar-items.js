initSidebarItems({"attr":[["gflags","Attribute macro for defining flags using the `bitflags` crate. This macro will also define a `GFlags::type_` function and the `glib::Value` traits."],["object_interface","Macro for boilerplate of `ObjectInterface` implementations."],["object_subclass","Macro for boilerplate of `ObjectSubclass` implementations."]],"constant":[["CLONE_MACRO_LOG_DOMAIN","This is the log domain used by the [`clone!`][crate::clone!] macro. If you want to use a custom logger (it prints to stdout by default), you can set your own logger using the corresponding `log` functions."]],"derive":[["Downgrade","Macro for deriving implementations of `glib::clone::Downgrade` and `glib::clone::Upgrade` traits and a weak type."],["GBoxed","Derive macro for defining a `BoxedType``::type_` function and the `glib::Value` traits."],["GEnum",""],["GErrorDomain","Derive macro for defining a GLib error domain and its associated `ErrorDomain` trait."],["GSharedBoxed","Derive macro for defining a `SharedType``::get_type` function and the `glib::Value` traits."]],"enum":[["ChecksumType","The hashing algorithm to be used by [`Checksum`][crate::Checksum] when performing the digest of some data."],["DateMonth","Enumeration representing a month; values are [`January`][Self::January], [`February`][Self::February], etc. [`BadMonth`][Self::BadMonth] is the invalid value."],["DateWeekday","Enumeration representing a day of the week; [`Monday`][Self::Monday], [`Tuesday`][Self::Tuesday], etc. [`BadWeekday`][Self::BadWeekday] is an invalid weekday."],["FileError",""],["GlibLoggerDomain","Enumeration of the possible domain handling behaviours for a `GlibLogger`."],["GlibLoggerFormat","Enumeration of the possible formatting behaviours for a `GlibLogger`."],["KeyFileError","Error codes returned by key file parsing."],["LogLevel",""],["OptionArg","The [`OptionArg`][crate::OptionArg] enum values determine which type of extra argument the options expect to find. If an option expects an extra argument, it can be specified in several ways; with a short option: `-x arg`, with a long option: `--name arg` or combined in a single argument: `--name=arg`."],["SeekType","An enumeration specifying the base position for a `g_io_channel_seek_position()` operation."],["TimeType","Disambiguates a given time in two ways."],["UriError","Error codes returned by [`Uri`][crate::Uri] methods."],["UserDirectory","These are logical ids for special directories which are defined depending on the platform used. You should use [`user_special_dir()`][crate::user_special_dir()] to retrieve the full path associated to the logical id."]],"fn":[["access","A wrapper for the POSIX `access()` function. This function is used to test a pathname for one or several of read, write or execute permissions, or just existence."],["application_name","Gets a human-readable name for the application, as set by [`set_application_name()`][crate::set_application_name()]. This name should be localized if possible, and is intended for display to the user. Contrast with `g_get_prgname()`, which gets a non-localized name. If [`set_application_name()`][crate::set_application_name()] has not been called, returns the result of `g_get_prgname()` (which may be [`None`] if `g_set_prgname()` has also not been called)."],["assert_warning",""],["assertion_message",""],["assertion_message_cmpstr",""],["base64_decode",""],["base64_encode",""],["bit_nth_lsf",""],["bit_nth_msf",""],["bit_storage",""],["build_filenamev",""],["build_pathv",""],["canonicalize_filename",""],["charset","Obtain the character set for the current locale."],["chdir",""],["check_version",""],["child_watch_future","Create a `Future` that will resolve once the child process with the given pid exits"],["child_watch_future_with_priority","Create a `Future` that will resolve once the child process with the given pid exits"],["clear_error",""],["codeset","Gets the character set for the current locale."],["compute_checksum_for_bytes",""],["compute_checksum_for_data",""],["compute_checksum_for_string",""],["compute_hmac_for_bytes",""],["compute_hmac_for_data",""],["compute_hmac_for_string",""],["console_charset","Obtains the character set used by the console attached to the process, which is suitable for printing output to the terminal."],["current_dir",""],["dcgettext",""],["dgettext",""],["dngettext",""],["dpgettext",""],["dpgettext2",""],["environ","Gets the list of environment variables for the current process."],["environ_getenv",""],["file_get_contents",""],["file_open_tmp",""],["file_read_link",""],["file_set_contents",""],["file_set_contents_full",""],["file_test",""],["filename_display_basename",""],["filename_display_name",""],["filename_from_uri",""],["filename_to_uri","Converts an absolute filename to an escaped ASCII-encoded URI, with the path component following Section 3.3. of RFC 2396."],["find_program_in_path","Locates the first executable named `program` in the user’s path, in the same way that `execvp()` would locate it. Returns an allocated string with the absolute path name, or [`None`] if the program is not found in the path. If `program` is already an absolute path, returns a copy of `program` if `program` exists and is executable, and [`None`] otherwise."],["format_size","Formats a size (for example the size of a file) into a human readable string. Sizes are rounded to the nearest size prefix (kB, MB, GB) and are displayed rounded to the nearest tenth. E.g. the file size 3292528 bytes will be converted into the string “3.2 MB”. The returned string is UTF-8, and may use a non-breaking space to separate the number and units, to ensure they aren’t separated when line wrapped."],["format_size_full","Formats a size."],["getenv","Returns the value of an environment variable."],["home_dir",""],["host_name","Return a name for the machine."],["hostname_is_ascii_encoded","Tests if `hostname` contains segments with an ASCII-compatible encoding of an Internationalized Domain Name. If this returns [`true`], you should decode the hostname with [`hostname_to_unicode()`][crate::hostname_to_unicode()] before displaying it to the user."],["hostname_is_ip_address","Tests if `hostname` is the string form of an IPv4 or IPv6 address. (Eg, “192.168.0.1”.)"],["hostname_is_non_ascii","Tests if `hostname` contains Unicode characters. If this returns [`true`], you need to encode the hostname with [`hostname_to_ascii()`][crate::hostname_to_ascii()] before using it in non-IDN-aware contexts."],["hostname_to_ascii","Converts `hostname` to its canonical ASCII form; an ASCII-only string containing no uppercase letters and not ending with a trailing dot."],["hostname_to_unicode","Converts `hostname` to its canonical presentation form; a UTF-8 string in Unicode normalization form C, containing no uppercase letters, no forbidden characters, and no ASCII-encoded segments, and not ending with a trailing dot."],["interval_stream","Create a `Stream` that will provide a value every given number of milliseconds."],["interval_stream_seconds","Create a `Stream` that will provide a value every given number of seconds."],["interval_stream_seconds_with_priority","Create a `Stream` that will provide a value every given number of seconds."],["interval_stream_with_priority","Create a `Stream` that will provide a value every given number of milliseconds."],["is_canonical_pspec_name",""],["language_names","Computes a list of applicable locale names, which can be used to e.g. construct locale-dependent filenames or search paths. The returned list is sorted from most desirable to least desirable and always contains the default locale “C”."],["language_names_with_category","Computes a list of applicable locale names with a locale category name, which can be used to construct the fallback locale-dependent filenames or search paths. The returned list is sorted from most desirable to least desirable and always contains the default locale “C”."],["listenv","Gets the names of all variables set in the environment."],["locale_variants","Returns a list of derived variants of `locale`, which can be used to e.g. construct locale-dependent filenames or search paths. The returned list is sorted from most desirable to least desirable. This function handles territory, charset and extra locale modifiers. See `setlocale(3)` for information about locales and their format."],["log_default_handler","The default log handler set up by GLib; `g_log_set_default_handler()` allows to install an alternate default log handler. This is used if no log handler has been set for the particular log domain and log level combination. It outputs the message to stderr or stdout and if the log level is fatal it calls G_BREAKPOINT(). It automatically prints a new-line character after the message, so one does not need to be manually included in `message`."],["log_remove_handler","Removes the log handler."],["log_set_always_fatal","Sets the message levels which are always fatal, in any log domain. When a message with any of these levels is logged the program terminates. You can only set the levels defined by GLib to be fatal. [`LogLevelFlags::LEVEL_ERROR`][crate::LogLevelFlags::LEVEL_ERROR] is always fatal."],["log_set_default_handler","To set back the default print handler, use the [`log_unset_default_handler`] function. Installs a default log handler which is used if no log handler has been set for the particular log domain and log level combination. By default, GLib uses `g_log_default_handler()` as default log handler."],["log_set_fatal_mask","Sets the log levels which are fatal in the given domain. [`LogLevelFlags::LEVEL_ERROR`][crate::LogLevelFlags::LEVEL_ERROR] is always fatal."],["log_set_handler","Sets the log handler for a domain and a set of log levels. To handle fatal and recursive messages the `log_levels` parameter must be combined with the [`LogLevelFlags::FLAG_FATAL`][crate::LogLevelFlags::FLAG_FATAL] and [`LogLevelFlags::FLAG_RECURSION`][crate::LogLevelFlags::FLAG_RECURSION] bit flags."],["log_unset_default_handler","To set the default print handler, use the [`log_set_default_handler`] function."],["main_current_source","Returns the currently firing source for this thread."],["main_depth","Returns the depth of the stack of calls to [`MainContext::dispatch()`][crate::MainContext::dispatch()] on any [`MainContext`][crate::MainContext] in the current thread. That is, when called from the toplevel, it gives 0. When called from within a callback from [`MainContext::iteration()`][crate::MainContext::iteration()] (or [`MainLoop::run()`][crate::MainLoop::run()], etc.) it returns 1. When called from within a callback to a recursive call to [`MainContext::iteration()`][crate::MainContext::iteration()], it returns 2. And so forth."],["markup_escape_text","Escapes text so that the markup parser will parse it verbatim. Less than, greater than, ampersand, etc. are replaced with the corresponding entities. This function would typically be used when writing out a file to be parsed with the markup parser."],["mkdir_with_parents","Create a directory if it doesn’t already exist. Create intermediate parent directories as needed, too."],["mkdtemp","Creates a temporary directory. See the `mkdtemp()` documentation on most UNIX-like systems."],["mkdtemp_full","Creates a temporary directory. See the `mkdtemp()` documentation on most UNIX-like systems."],["mkstemp","Opens a temporary file. See the `mkstemp()` documentation on most UNIX-like systems."],["mkstemp_full","Opens a temporary file. See the `mkstemp()` documentation on most UNIX-like systems."],["monotonic_time","Queries the system monotonic time."],["num_processors","Determine the approximate number of threads that the system will schedule simultaneously for this process. This is intended to be used as a parameter to `g_thread_pool_new()` for CPU bound tasks and similar cases."],["on_error_query","Prompts the user with `[E]xit, [H]alt, show [S]tack trace or [P]roceed`. This function is intended to be used for debugging use only. The following example shows how it can be used together with the `g_log()` functions."],["on_error_stack_trace","Invokes gdb, which attaches to the current process and shows a stack trace. Called by [`on_error_query()`][crate::on_error_query()] when the “[S]tack trace” option is selected. You can get the current process’s program name with `g_get_prgname()`, assuming that you have called `gtk_init()` or `gdk_init()`."],["os_info","Get information about the operating system."],["path_get_basename","Gets the last component of the filename."],["path_get_dirname","Gets the directory components of a file name. For example, the directory component of `/usr/bin/test` is `/usr/bin`. The directory component of `/` is `/`."],["path_is_absolute","Returns [`true`] if the given `file_name` is an absolute file name. Note that this is a somewhat vague concept on Windows."],["path_skip_root","Returns a pointer into `file_name` after the root component, i.e. after the “/” in UNIX or “C:\" under Windows. If `file_name` is not an absolute path it returns [`None`]."],["pattern_match_simple","Matches a string against a pattern given as a string. If this function is to be called in a loop, it’s more efficient to compile the pattern once with `g_pattern_spec_new()` and call [`pattern_match_string()`][crate::pattern_match_string()] repeatedly."],["prgname",""],["program_name","Same as `get_prgname()`."],["random_double","Returns a random `gdouble` equally distributed over the range [0..1)."],["random_double_range","Returns a random `gdouble` equally distributed over the range [`begin`..`end`)."],["random_int","Return a random `guint32` equally distributed over the range [0..2^32-1]."],["random_int_range","Returns a random `gint32` equally distributed over the range [`begin`..`end`-1]."],["random_set_seed","Sets the seed for the global random number generator, which is used by the g_random_* functions, to `seed`."],["real_name",""],["real_time","Queries the system wall-clock time."],["reload_user_special_dirs_cache","Resets the cache used for [`user_special_dir()`][crate::user_special_dir()], so that the latest on-disk version is used. Call this only if you just changed the data on disk yourself."],["return_if_fail_warning","Internal function used to print messages from the public `g_return_if_fail()` and `g_return_val_if_fail()` macros."],["rmdir","A wrapper for the POSIX `rmdir()` function. The `rmdir()` function deletes a directory from the filesystem."],["rust_log_handler","Provides a glib log handler which routes all logging messages to the `log crate`."],["set_application_name","Sets a human-readable name for the application. This name should be localized if possible, and is intended for display to the user. Contrast with `g_set_prgname()`, which sets a non-localized name. `g_set_prgname()` will be called automatically by `gtk_init()`, but [`set_application_name()`][crate::set_application_name()] will not."],["set_prgname","Sets the name of the program. This name should not be localized, in contrast to [`set_application_name()`][crate::set_application_name()]."],["set_print_handler","To set back the default print handler, use the [`unset_print_handler`] function. Sets the print handler."],["set_printerr_handler","To set back the default print handler, use the [`unset_printerr_handler`] function. Sets the handler for printing error messages."],["set_program_name","Same as `set_prgname()`."],["setenv","Sets an environment variable. On UNIX, both the variable’s name and value can be arbitrary byte strings, except that the variable’s name cannot contain ‘=’. On Windows, they should be in UTF-8."],["shell_parse_argv","Parses a command line into an argument vector, in much the same way the shell would, but without many of the expansions the shell would perform (variable expansion, globs, operators, filename expansion, etc. are not supported). The results are defined to be the same as those you would get from a UNIX98 /bin/sh, as long as the input contains none of the unsupported shell expansions. If the input does contain such expansions, they are passed through literally. Possible errors are those from the `G_SHELL_ERROR` domain. Free the returned vector with `g_strfreev()`."],["shell_quote","Quotes a string so that the shell (/bin/sh) will interpret the quoted string to mean `unquoted_string`. If you pass a filename to the shell, for example, you should first quote it with this function. The return value must be freed with `g_free()`. The quoting style used is undefined (single or double quotes may be used)."],["shell_unquote","Unquotes a string as the shell (/bin/sh) would. Only handles quotes; if a string contains file globs, arithmetic operators, variables, backticks, redirections, or other special-to-the-shell features, the result will be different from the result a real shell would produce (the variables, backticks, etc. will be passed through literally instead of being expanded). This function is guaranteed to succeed if applied to the result of [`shell_quote()`][crate::shell_quote()]. If it fails, it returns [`None`] and sets the error. The `quoted_string` need not actually contain quoted or escaped text; [`shell_unquote()`][crate::shell_unquote()] simply goes through the string and unquotes/unescapes anything that the shell would. Both single and double quotes are handled, as are escapes including escaped newlines. The return value must be freed with `g_free()`. Possible errors are in the `G_SHELL_ERROR` domain."],["spaced_primes_closest","Gets the smallest prime number from a built-in array of primes which is larger than `num`. This is used within GLib to calculate the optimum size of a `GHashTable`."],["spawn_async","See `g_spawn_async_with_pipes()` for a full description; this function simply calls the `g_spawn_async_with_pipes()` without any pipes."],["spawn_async_with_fds","Identical to `g_spawn_async_with_pipes_and_fds()` but with `n_fds` set to zero, so no FD assignments are used."],["spawn_async_with_pipes","Identical to `g_spawn_async_with_pipes_and_fds()` but with `n_fds` set to zero, so no FD assignments are used."],["spawn_check_exit_status","Set `error` if `exit_status` indicates the child exited abnormally (e.g. with a nonzero exit code, or via a fatal signal)."],["spawn_command_line_async","A simple version of [`spawn_async()`][crate::spawn_async()] that parses a command line with [`shell_parse_argv()`][crate::shell_parse_argv()] and passes it to [`spawn_async()`][crate::spawn_async()]. Runs a command line in the background. Unlike [`spawn_async()`][crate::spawn_async()], the [`SpawnFlags::SEARCH_PATH`][crate::SpawnFlags::SEARCH_PATH] flag is enabled, other flags are not. Note that [`SpawnFlags::SEARCH_PATH`][crate::SpawnFlags::SEARCH_PATH] can have security implications, so consider using [`spawn_async()`][crate::spawn_async()] directly if appropriate. Possible errors are those from [`shell_parse_argv()`][crate::shell_parse_argv()] and [`spawn_async()`][crate::spawn_async()]."],["stpcpy","Copies a nul-terminated string into the dest buffer, include the trailing nul, and return a pointer to the trailing nul byte. This is useful for concatenating multiple strings together without having to repeatedly scan for the end."],["system_config_dirs","Returns an ordered list of base directories in which to access system-wide configuration information."],["system_data_dirs","Returns an ordered list of base directories in which to access system-wide application data."],["timeout_future","Create a `Future` that will resolve after the given number of milliseconds."],["timeout_future_seconds","Create a `Future` that will resolve after the given number of seconds."],["timeout_future_seconds_with_priority","Create a `Future` that will resolve after the given number of seconds."],["timeout_future_with_priority","Create a `Future` that will resolve after the given number of milliseconds."],["tmp_dir",""],["unix_open_pipe","Similar to the UNIX `pipe()` call, but on modern systems like Linux uses the `pipe2()` system call, which atomically creates a pipe with the configured flags. The only supported flag currently is `FD_CLOEXEC`. If for example you want to configure `O_NONBLOCK`, that must still be done separately with `fcntl()`."],["unix_signal_future","Create a `Future` that will resolve once the given UNIX signal is raised"],["unix_signal_future_with_priority","Create a `Future` that will resolve once the given UNIX signal is raised"],["unix_signal_stream","Create a `Stream` that will provide a value whenever the given UNIX signal is raised"],["unix_signal_stream_with_priority","Create a `Stream` that will provide a value whenever the given UNIX signal is raised"],["unlink","A wrapper for the POSIX `unlink()` function. The `unlink()` function deletes a name from the filesystem. If this was the last link to the file and no processes have it opened, the diskspace occupied by the file is freed."],["unset_print_handler","To set the default print handler, use the [`set_print_handler`] function."],["unset_printerr_handler","To set the default print handler, use the [`set_printerr_handler`] function."],["unsetenv","Removes an environment variable from the environment."],["user_cache_dir","Returns a base directory in which to store non-essential, cached data specific to particular user."],["user_config_dir","Returns a base directory in which to store user-specific application configuration information such as user preferences and settings."],["user_data_dir","Returns a base directory in which to access application data such as icons that is customized for a particular user."],["user_name",""],["user_runtime_dir","Returns a directory that is unique to the current user on the local system."],["user_special_dir","Returns the full path of a special directory using its logical id."],["usleep","Pauses the current thread for the given number of microseconds."],["uuid_string_is_valid","Parses the string `str` and verify if it is a UUID."],["uuid_string_random","Generates a random UUID (RFC 4122 version 4) as a string. It has the same randomness guarantees as `GRand`, so must not be used for cryptographic purposes such as key generation, nonces, salts or one-time pads."],["warn_message","Internal function used to print messages from the public `g_warn_if_reached()` and `g_warn_if_fail()` macros."]],"macro":[["bool_error","Generic error used for functions that fail without any further information"],["clone","Macro for passing variables as strong or weak references into a closure."],["debug","A macro which behaves exactly as `log::debug!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["error","A macro which behaves exactly as `log::error!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["g_critical","Macro used to log using GLib logging system. It uses g_log."],["g_debug","Macro used to log using GLib logging system. It uses g_log."],["g_error","Macro used to log using GLib logging system. It uses g_log."],["g_info","Macro used to log using GLib logging system. It uses g_log."],["g_log","Macro used to log using GLib logging system. It uses g_log."],["g_message","Macro used to log using GLib logging system. It uses g_log."],["g_print","Macro used to print messages. It uses g_print."],["g_printerr","Macro used to print error messages. It uses g_printerr."],["g_warning","Macro used to log using GLib logging system. It uses g_log."],["glib_boxed_wrapper","Wrapper implementations for Boxed types. See `wrapper!`."],["glib_object_wrapper","ObjectType implementations for Object types. See `wrapper!`."],["glib_shared_wrapper","Wrapper implementations for shared types. See `wrapper!`."],["info","A macro which behaves exactly as `log::info!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["result_from_gboolean",""],["trace","A macro which behaves exactly as `log::trace!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["warn","A macro which behaves exactly as `log::warn!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["wrapper","Defines a wrapper type and implements the appropriate traits."]],"mod":[["boxed","`IMPL` Boxed wrapper implementation."],["char",""],["clone",""],["closure",""],["error","`Error` binding and helper trait."],["functions",""],["object","`IMPL` Object wrapper implementation and `Object` binding."],["prelude","Traits and essential types intended for blanket imports."],["send_unique",""],["shared","`IMPL` Shared (reference counted) wrapper implementation."],["signal","`IMPL` Low level signal support."],["source",""],["subclass","Module containing infrastructure for subclassing `GObject`s and registering boxed types."],["translate","Translation between GLib/GLib-based FFI types and their Rust counterparts."],["types","Runtime type information."],["value","`Value` binding and helper traits."],["variant","`Variant` binding and helper traits."],["wrapper","`IMPL` The `wrapper!` macro and miscellaneous wrapper traits."]],"static":[["CSET_A_2_Z","The set of uppercase ASCII alphabet characters. Used for specifying valid identifier characters in `GScannerConfig`."],["CSET_DIGITS","The set of ASCII digits. Used for specifying valid identifier characters in `GScannerConfig`."],["CSET_a_2_z","The set of lowercase ASCII alphabet characters. Used for specifying valid identifier characters in `GScannerConfig`."],["KEY_FILE_DESKTOP_GROUP","The name of the main group of a desktop entry file, as defined in the Desktop Entry Specification. Consult the specification for more details about the meanings of the keys below."],["KEY_FILE_DESKTOP_KEY_ACTIONS","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a string list giving the available application actions."],["KEY_FILE_DESKTOP_KEY_CATEGORIES","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a list of strings giving the categories in which the desktop entry should be shown in a menu."],["KEY_FILE_DESKTOP_KEY_COMMENT","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a localized string giving the tooltip for the desktop entry."],["KEY_FILE_DESKTOP_KEY_DBUS_ACTIVATABLE","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a boolean set to true if the application is D-Bus activatable."],["KEY_FILE_DESKTOP_KEY_EXEC","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a string giving the command line to execute. It is only valid for desktop entries with the `Application` type."],["KEY_FILE_DESKTOP_KEY_GENERIC_NAME","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a localized string giving the generic name of the desktop entry."],["KEY_FILE_DESKTOP_KEY_HIDDEN","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a boolean stating whether the desktop entry has been deleted by the user."],["KEY_FILE_DESKTOP_KEY_ICON","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a localized string giving the name of the icon to be displayed for the desktop entry."],["KEY_FILE_DESKTOP_KEY_MIME_TYPE","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a list of strings giving the MIME types supported by this desktop entry."],["KEY_FILE_DESKTOP_KEY_NAME","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a localized string giving the specific name of the desktop entry."],["KEY_FILE_DESKTOP_KEY_NOT_SHOW_IN","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a list of strings identifying the environments that should not display the desktop entry."],["KEY_FILE_DESKTOP_KEY_NO_DISPLAY","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a boolean stating whether the desktop entry should be shown in menus."],["KEY_FILE_DESKTOP_KEY_ONLY_SHOW_IN","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a list of strings identifying the environments that should display the desktop entry."],["KEY_FILE_DESKTOP_KEY_PATH","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a string containing the working directory to run the program in. It is only valid for desktop entries with the `Application` type."],["KEY_FILE_DESKTOP_KEY_STARTUP_NOTIFY","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a boolean stating whether the application supports the Startup Notification Protocol Specification."],["KEY_FILE_DESKTOP_KEY_STARTUP_WM_CLASS","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is string identifying the WM class or name hint of a window that the application will create, which can be used to emulate Startup Notification with older applications."],["KEY_FILE_DESKTOP_KEY_TERMINAL","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a boolean stating whether the program should be run in a terminal window. It is only valid for desktop entries with the `Application` type."],["KEY_FILE_DESKTOP_KEY_TRY_EXEC","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a string giving the file name of a binary on disk used to determine if the program is actually installed. It is only valid for desktop entries with the `Application` type."],["KEY_FILE_DESKTOP_KEY_TYPE","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a string giving the type of the desktop entry. Usually [`KEY_FILE_DESKTOP_TYPE_APPLICATION`][crate::KEY_FILE_DESKTOP_TYPE_APPLICATION], [`KEY_FILE_DESKTOP_TYPE_LINK`][crate::KEY_FILE_DESKTOP_TYPE_LINK], or [`KEY_FILE_DESKTOP_TYPE_DIRECTORY`][crate::KEY_FILE_DESKTOP_TYPE_DIRECTORY]."],["KEY_FILE_DESKTOP_KEY_URL","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a string giving the URL to access. It is only valid for desktop entries with the `Link` type."],["KEY_FILE_DESKTOP_KEY_VERSION","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a string giving the version of the Desktop Entry Specification used for the desktop entry file."],["KEY_FILE_DESKTOP_TYPE_APPLICATION","The value of the [`KEY_FILE_DESKTOP_KEY_TYPE`][crate::KEY_FILE_DESKTOP_KEY_TYPE], key for desktop entries representing applications."],["KEY_FILE_DESKTOP_TYPE_DIRECTORY","The value of the [`KEY_FILE_DESKTOP_KEY_TYPE`][crate::KEY_FILE_DESKTOP_KEY_TYPE], key for desktop entries representing directories."],["KEY_FILE_DESKTOP_TYPE_LINK","The value of the [`KEY_FILE_DESKTOP_KEY_TYPE`][crate::KEY_FILE_DESKTOP_KEY_TYPE], key for desktop entries representing links to documents."],["OPTION_REMAINING","If a long option in the main group has this name, it is not treated as a regular option. Instead it collects all non-option arguments which would otherwise be left in `argv`. The option must be of type [`OptionArg::Callback`][crate::OptionArg::Callback], [`OptionArg::StringArray`][crate::OptionArg::StringArray] or [`OptionArg::FilenameArray`][crate::OptionArg::FilenameArray]."],["STR_DELIMITERS","The standard delimiters, used in `g_strdelimit()`."],["TEST_OPTION_ISOLATE_DIRS","Creates a unique temporary directory for each unit test and uses `g_set_user_dirs()` to set XDG directories to point into subdirectories of it for the duration of the unit test. The directory tree is cleaned up after the test finishes successfully. Note that this doesn’t take effect until `g_test_run()` is called, so calls to (for example) `g_get_user_home_dir()` will return the system-wide value when made in a test program’s `main()` function."],["URI_RESERVED_CHARS_GENERIC_DELIMITERS","Generic delimiters characters as defined in RFC 3986. Includes `:/?#[]@`."],["URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS","Subcomponent delimiter characters as defined in RFC 3986. Includes `!$&'()*+,;=`."],["g_param_spec_types",""]],"struct":[["Array",""],["Binding",""],["BindingFlags","Flags to be passed to [`ObjectExtManual::bind_property()`][crate::prelude::ObjectExtManual::bind_property()] or [`ObjectExtManual::bind_property_full()`][crate::prelude::ObjectExtManual::bind_property_full()]."],["ByteArray","Contains the public fields of a GByteArray."],["Bytes","A shared immutable byte slice (the equivalent of `Rc<[u8]>`)."],["Checksum","An opaque structure representing a checksumming operation. To create a new GChecksum, use [`new()`][Self::new()]. To free a GChecksum, use `g_checksum_free()`."],["Date",""],["DateTime","`GDateTime` is an opaque structure whose members cannot be accessed directly."],["EnumClass","Representation of an `enum` for dynamically, at runtime, querying the values of the enum and using them."],["EnumValue","Representation of a single enum value of an `EnumClass`."],["FileSetContentsFlags","Flags to pass to [`file_set_contents_full()`][crate::file_set_contents_full()] to affect its safety and performance."],["FileTest","A test to perform on a file using [`file_test()`][crate::file_test()]."],["FlagsBuilder","Builder for conveniently setting/unsetting flags and returning a `Value`."],["FlagsClass","Representation of a `flags` for dynamically, at runtime, querying the values of the enum and using them"],["FlagsValue","Representation of a single flags value of a `FlagsClass`."],["FormatSizeFlags","Flags to modify the format of the string returned by [`format_size_full()`][crate::format_size_full()]."],["GString",""],["GlibLogger","An implementation of a `log` compatible logger which logs over glib logging facilities."],["IOCondition","A bitwise combination representing a condition to watch for on an event source."],["KeyFile","The GKeyFile struct contains only private data and should not be accessed directly."],["KeyFileFlags","Flags which influence the parsing."],["LogHandlerId",""],["LogLevelFlags","Flags specifying the level of log messages."],["LogLevels",""],["MainContext","The `GMainContext` struct is an opaque data type representing a set of sources to be handled in a main loop."],["MainLoop","The `GMainLoop` struct is an opaque data type representing the main event loop of a GLib or GTK+ application."],["OptionFlags","Flags which modify individual options."],["ParamFlags","Through the [`ParamFlags`][crate::ParamFlags] flag values, certain aspects of parameters can be configured. See also `G_PARAM_STATIC_STRINGS`."],["ParamSpec",""],["ParamSpecBoolean",""],["ParamSpecBoxed",""],["ParamSpecChar",""],["ParamSpecDouble",""],["ParamSpecEnum",""],["ParamSpecFlags",""],["ParamSpecFloat",""],["ParamSpecGType",""],["ParamSpecInt",""],["ParamSpecInt64",""],["ParamSpecLong",""],["ParamSpecObject",""],["ParamSpecOverride",""],["ParamSpecParam",""],["ParamSpecPointer",""],["ParamSpecString",""],["ParamSpecUChar",""],["ParamSpecUInt",""],["ParamSpecUInt64",""],["ParamSpecULong",""],["ParamSpecUnichar",""],["ParamSpecValueArray",""],["ParamSpecVariant",""],["Quark",""],["Receiver","A `Receiver` that can be attached to a main context to receive items from its corresponding `Sender` or `SyncSender`."],["Sender","A `Sender` that can be used to send items to the corresponding main context receiver."],["SignalFlags","The signal flags are used to specify a signal’s behaviour, the overall signal description outlines how especially the RUN flags control the stages of a signal emission."],["Source","The `GSource` struct is an opaque data type representing an event source."],["SourceFuture","Represents a `Future` around a `glib::Source`. The future will be resolved once the source has provided a value"],["SourceStream","Represents a `Stream` around a `glib::Source`. The stream will be provide all values that are provided by the source"],["SpawnFlags","Flags passed to [`spawn_sync()`][crate::spawn_sync()], [`spawn_async()`][crate::spawn_async()] and `g_spawn_async_with_pipes()`."],["String","A mutable text buffer that grows automatically."],["SyncSender","A `SyncSender` that can be used to send items to the corresponding main context receiver."],["ThreadPool",""],["TimeZone","[`TimeZone`][crate::TimeZone] is an opaque structure whose members cannot be accessed directly."],["Uri","The [`Uri`][crate::Uri] type and related functions can be used to parse URIs into their components, and build valid URIs from individual components."],["UriFlags","Flags that describe a URI."],["UriHideFlags","Flags describing what parts of the URI to hide in [`Uri::to_string_partial()`][crate::Uri::to_string_partial()]. Note that [`PASSWORD`][Self::PASSWORD] and [`AUTH_PARAMS`][Self::AUTH_PARAMS] will only work if the [`Uri`][crate::Uri] was parsed with the corresponding flags."],["UriParamsFlags","Flags modifying the way parameters are handled by [`Uri::parse_params()`][crate::Uri::parse_params()] and `GUriParamsIter`."],["ValueArray",""],["VariantDict","`VariantDict` is a mutable key/value store where the keys are always strings and the values are `Variant`s."],["VariantIter","Iterator over items in a variant."],["VariantTy","Describes `Variant` types."],["VariantType","Describes `Variant` types."]],"trait":[["ParamSpecType",""]],"type":[["DateDay",""],["DateYear",""],["Time",""],["TimeSpan",""]]});