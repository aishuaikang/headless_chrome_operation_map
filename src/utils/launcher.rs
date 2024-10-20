use std::path::{Path, PathBuf};
use std::{env, fs};

fn _look_path(input: &str) -> Option<PathBuf> {
    // 如果输入是一个完整的路径，直接检查该路径
    if let Some(path) = Path::new(input).canonicalize().ok() {
        if fs::metadata(&path).ok()?.is_file() {
            return Some(path);
        }
    }

    // 获取 PATH 环境变量
    let paths = env::var("PATH").ok()?;

    // 根据操作系统选择路径分隔符
    let separator = if cfg!(target_os = "windows") {
        ';'
    } else {
        ':'
    };

    // 可能的扩展名
    let extensions = if cfg!(target_os = "windows") {
        vec!["exe", "com", "bat", "cmd"]
    } else {
        vec![]
    };

    // 搜索每个目录
    for path in paths.split(separator) {
        let dir = Path::new(path);

        // 构建可能的命令路径
        let mut possible_paths = vec![dir.join(input)];

        if cfg!(target_os = "windows") {
            for ext in &extensions {
                possible_paths.push(dir.join(format!("{}.{}", input, ext)));
            }
        }

        for command_path in possible_paths {
            // 检查文件是否存在且可执行
            if let Ok(metadata) = fs::metadata(&command_path) {
                if metadata.is_file() {
                    return Some(command_path.canonicalize().ok()?);
                }
            }
        }
    }

    None
}
pub fn look_path() -> Option<PathBuf> {
    let list = match env::consts::OS {
        "macos" => vec![
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
            "/Applications/Google Chrome Canary.app/Contents/MacOS/Google Chrome Canary",
            "/usr/bin/google-chrome-stable",
            "/usr/bin/google-chrome",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>(),
        "linux" => vec![
            "chrome",
            "google-chrome",
            "/usr/bin/google-chrome",
            "microsoft-edge",
            "/usr/bin/microsoft-edge",
            "chromium",
            "chromium-browser",
            "/usr/bin/google-chrome-stable",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
            "/snap/bin/chromium",
            "/data/data/com.termux/files/usr/bin/chromium-browser",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>(),
        "openbsd" => vec!["chrome", "chromium"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        "windows" => {
            let mut paths = vec!["chrome", "edge"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            paths.extend(expand_windows_exe_paths(vec![
                r"Google\Chrome\Application\chrome.exe",
                r"Chromium\Application\chrome.exe",
                r"Microsoft\Edge\Application\msedge.exe",
            ]));
            paths
        }
        _ => vec![],
    };

    for path in list {
        if let Some(p) = _look_path(&path) {
            return Some(p);
        }
    }

    None
}

fn expand_windows_exe_paths(list: Vec<&str>) -> Vec<String> {
    let mut new_list = Vec::new();
    for p in list {
        if let Some(program_files) = env::var_os("ProgramFiles") {
            new_list.push(
                PathBuf::from(&program_files)
                    .join(p)
                    .to_string_lossy()
                    .to_string(),
            );
        }
        if let Some(program_files_x86) = env::var_os("ProgramFiles(x86)") {
            new_list.push(
                PathBuf::from(&program_files_x86)
                    .join(p)
                    .to_string_lossy()
                    .to_string(),
            );
        }
        if let Some(local_app_data) = env::var_os("LocalAppData") {
            new_list.push(
                PathBuf::from(&local_app_data)
                    .join(p)
                    .to_string_lossy()
                    .to_string(),
            );
        }
    }
    new_list
}
