// 模块声明
pub mod models;
pub mod scanners;
pub mod commands;
pub mod utils;
pub mod managers;

use commands::{
    scan_npm, scan_cargo, scan_pip, scan_dotfiles,
    update_tool, uninstall_tool, batch_update_tools, batch_uninstall_tools,
    read_config_file, write_config_file, list_json_files, list_json_files_recursive, get_home_path,
    search_npm_packages, search_cargo_packages, search_pip_packages,
    install_npm_package, install_cargo_package, install_pip_package,
    get_npm_versions, install_npm_version,
    scan_caches, clear_npm_cache, clear_pnpm_cache, clear_yarn_cache,
    clear_cargo_cache, clear_pip_cache,
    scan_ports, kill_process,
    get_runtime_versions,
    scan_disk_usage, get_dir_details,
    scan_orphan_dependencies,
    get_proxy_configs, set_proxy_config,
    get_env_variables, get_path_entries,
    scan_dev_processes,
    get_project_templates, create_project
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scan_npm,
            scan_cargo,
            scan_pip,
            scan_dotfiles,
            update_tool,
            uninstall_tool,
            batch_update_tools,
            batch_uninstall_tools,
            read_config_file,
            write_config_file,
            list_json_files,
            list_json_files_recursive,
            get_home_path,
            search_npm_packages,
            search_cargo_packages,
            search_pip_packages,
            install_npm_package,
            install_cargo_package,
            install_pip_package,
            get_npm_versions,
            install_npm_version,
            scan_caches,
            clear_npm_cache,
            clear_pnpm_cache,
            clear_yarn_cache,
            clear_cargo_cache,
            clear_pip_cache,
            scan_ports,
            kill_process,
            get_runtime_versions,
            scan_disk_usage,
            get_dir_details,
            scan_orphan_dependencies,
            get_proxy_configs,
            set_proxy_config,
            get_env_variables,
            get_path_entries,
            scan_dev_processes,
            get_project_templates,
            create_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
