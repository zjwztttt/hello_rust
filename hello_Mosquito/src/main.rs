use clipboard::{ClipboardContext, ClipboardProvider};
use std::process::Command;
use std::path::{Path, PathBuf};
use toml::Value;
use url::Url;
use std::env;
use std::fs;

/// 封装 cmd pause 功能，避免重复代码
fn pause() {
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

/// 根据视频标题，查找实际存在的视频文件（精准匹配）
fn find_existing_video(download_path: &Path, title: &str) -> Option<PathBuf> {
    let base = download_path.join(title);
    // 按优先级检查常见视频格式
    let exts = ["mp4", "webm", "mkv", "mov", "flv"];
    for ext in exts {
        let file = base.with_extension(ext);
        if file.exists() {
            return Some(file);
        }
    }
    None
}

/// 打开 Download 文件夹
fn open_folder_and_select_file(folder_path: impl AsRef<Path>) {
    let _ = Command::new("explorer.exe")
        .arg("/select,")
        .arg(folder_path.as_ref())
        .spawn();
}

fn main() {
    // ==================== 1. 设置基本信息 ====================
    // 获取当前程序所在目录
    let exe_path = match env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ 获取程序路径失败：{}", e);
            pause();
            return;
        }
    };

    let dqpath = match exe_path.parent() {
        Some(p) => p.to_path_buf(),
        None => {
            eprintln!("❌ 无法获取程序所在目录");
            pause();
            return;
        }
    };

    // ==================== 2. 检查并读取配置文件 ====================
    let setting_path = dqpath.join("setting.toml");
    if !setting_path.exists() {
        eprintln!("❌ 检测到程序还未安装,请以管理员身份运行 mosquito_install.exe 安装程序");
        pause();
        return;
    }
    // 读取配置文件内容
    let setting_text = match fs::read_to_string(&setting_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("❌ 读取配置文件失败：{}", e);
            pause();
            return;
        }
    };
    // 解析配置文件
    let config = match setting_text.parse::<Value>() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ 解析配置文件失败：{}", e);
            pause();
            return;
        }
    };
    // 直接拿到网站数组
    let sites_list = match config["general"]["supported"].as_array() {
        Some(arr) => arr,
        None => {
            eprintln!("❌ 配置文件中未找到支持的网站列表");
            pause();
            return;
        }
    };

    // ==================== 3. 创建下载目录 ====================
    let download_path = dqpath.join("Download");
    // 如果不存在，则创建 Download 文件夹
    if !download_path.exists() {
        if let Err(e) = fs::create_dir_all(&download_path) {
            eprintln!("❌ 创建下载目录失败：{}", e);
            pause();
            return;
        }
        println!("✅ 已成功创建 Download 文件夹");
    } else {
        println!("ℹ️ 已检测到 Download 文件夹");
    }

    // ==================== 4. 获取剪贴板链接 ====================
    let mut ctx = match ClipboardContext::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ 剪贴板初始化失败：{}", e);
            pause();
            return;
        }
    };

    // 2. 读取剪贴板内容
    let content_text = match ctx.get_contents() {
        Ok(c) => c.trim().to_string(), // 自动去空白符
        //Ok(c) => c,
        Err(e) => {
            eprintln!("❌ 读取剪贴板失败：{}", e);
            pause();
            return;
        }
    };

    // 释放剪贴板资源，避免后续命令行程序无法访问剪贴板
    drop(ctx);

    if content_text.is_empty() {
        eprintln!("❌ 剪贴板为空");
        pause();
        return;
    }

    // ==================== 5. 解析并校验 URL ====================
    let url = match Url::parse(&content_text) {
        Ok(u) => u,
        Err(_) => {
            eprintln!("❌ 剪贴板内容不是合法的 URL");
            pause();
            return;
        }
    };

    println!("✅ 正在解析视频下载链接！");
    println!("协议：{}", url.scheme());
    let domain = url.domain().unwrap_or_default();
    println!("域名：{}", domain);

    // 校验是否支持该网站
    let is_supported = sites_list.iter().any(|s| s.as_str().is_some_and(|site| domain.contains(site)));
    if !is_supported {
        eprintln!("❌ 不支持该网站：{}", domain);
        pause();
        return;
    }

    // ==================== 6. 执行下载 ====================
    println!("\n正在解析并下载视频,请稍候...\n");

    // YouTube 下载
    if domain.contains("youtube.com") || domain.contains("youtu.be") {
        // 检查 yt_dlp.exe 下载程序是否存在
        let yt_dlp_path = dqpath.join("yt-dlp.exe");
        if !yt_dlp_path.exists() {
            eprintln!("❌ 未找到 yt-dlp.exe,请放置在:{}", dqpath.to_string_lossy());
            pause();
            return;
        }

        // 获取视频标题（用于判断文件是否存在）
        let title_output = Command::new(&yt_dlp_path)
            .arg("--get-title")
            .arg("--no-playlist")
            .arg(&content_text)
            .output();

        let video_title = match title_output {
            Ok(out) if out.status.success() => {
                String::from_utf8_lossy(&out.stdout).trim().to_string()
            }
            _ => "unknown_video".to_string(),
        };
        if let Some(video_file) = find_existing_video(&download_path, &video_title) {
            println!("\n✅ 视频已经存在，无需重复下载！");
            open_folder_and_select_file(&video_file);
            pause();
            return;
        }
        println!("✅ 本次下载由 yt-dlp 提供");

        //执行命令：yt_dlp.exe -o 保存路径 网址
        let output1 = Command::new(&yt_dlp_path)
            // 核心：不检查更新，直接跳过 403 限流错误
            .arg("--no-update")
            // 1. 只下载当前视频，不下载播放列表（解决你最后那个大错误）
            .arg("--no-playlist")
            // 👇 关键：强制走你的代理（必须填你自己的端口）
            //.arg("--proxy")
            //.arg("auto")
            // 必须的 JS 运行时
            //.arg("--js-runtimes").arg("deno")
            // 2. 多客户端轮换（tv→ios→android_vr，绕过限制）
            //.arg("--extractor-args")
            //.arg("youtube:player_client=tv,ios,android_vr")
            // 3. 带 Cookie（解决登录/年龄/地区限制）
            //.arg("--cookies").arg(cookie_file)
            // ✅ 自动下载最高清晰度（不会报格式不可用）
            //.arg("-f").arg("bestvideo+bestaudio/best")
            //.arg("--merge-output-format").arg("mp4")
            // 👈 强制覆盖已存在文件
            //.arg("--force-overwrites")
            // 👈 强制重命名（如果文件已存在，自动在文件名后添加数字避免覆盖）
            //.arg("--no-overwrites")
            //.arg("--continue")
            // 指定下载目录+自动标题命名
            .arg("-o")
            .arg(download_path.join("%(title)s.%(ext)s").as_os_str())
            // 视频链接
            .arg(&content_text)
            // 执行并打印日志
            .status();
        match output1 {
            Ok(s) if s.success() => {
                if let Some(video_file) = find_existing_video(&download_path, &video_title) {
                    println!("✅ 下载完成！即将打开文件夹并选中视频");
                    // 打开下载目录并且选中下载的视频
                    open_folder_and_select_file(&video_file);
                }
            }
            Ok(s) => eprintln!("❌ 下载失败，退出码：{:?}", s.code()),
            Err(e) => eprintln!("❌ 启动下载失败：{}", e),
        }
    } else {// 其他网站使用 lux 下载
        // 检查 lux.exe 下载程序是否存在
        let lux_path = dqpath.join("lux.exe");
        if !lux_path.exists() {
            eprintln!("❌ 未找到 lux.exe，请放置在：{}", dqpath.to_string_lossy());
            pause();
            return;
        }
        // 获取视频标题（用于判断文件是否存在）
        let title_output = Command::new(&lux_path)
            .arg("-j")  // 输出 JSON 信息
            .arg(&content_text)
            .output();

        //let title_output1 = title_output.as_ref().map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_else(|_| "unknown".into());
        //println!("title_output1的值是{}", title_output1);
        let mut video_title = "unknown_video".to_string();
        if let Ok(out) = &title_output {
            if out.status.success() {
                // 1. 解析整个输出为 JSON Value
                if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&out.stdout) {
                    // 2. 关键：lux 输出是数组 []，必须取 [0] 第一个元素
                    if let Some(first_item) = json.get(0) {
                        // 3. 从第一个元素里取 title
                        if let Some(title) = first_item["title"].as_str() {
                            video_title = title.trim().to_string();
                        }
                    }
                }
            }
        }
        if let Some(video_file) = find_existing_video(&download_path, &video_title) {
            println!("\n✅ 视频已经存在，无需重复下载！");
            open_folder_and_select_file(&video_file);
            pause();
            return;
        }
        println!("✅ 本次下载由 LUX 提供");

        //执行命令：lux.exe -o 保存路径 网址
        let output = Command::new(&lux_path)
            //.arg("--overwrite")  // 强制覆盖已存在文件
            //.arg("--rename")   // 强制重命名（如果文件已存在，自动在文件名后添加数字避免覆盖）
            .arg("-o")          // 输出目录参数
            .arg(&download_path)      // 保存路径 %lj%
            .arg(&content_text)          // 视频链接 %URL%
            .status();                  // 执行并打印日志
        match output {
            Ok(s) if s.success() => {
                if let Some(video_file) = find_existing_video(&download_path, &video_title) {
                    println!("✅ 下载完成！即将打开文件夹并选中视频");
                    // lux 下载后会生成单个视频，自动打开目录
                    open_folder_and_select_file(&video_file);
                }
            }
            Ok(s) => eprintln!("❌ 下载失败，退出码：{:?}", s.code()),
            Err(e) => eprintln!("❌ 启动下载失败：{}", e),
        }
    }
    pause();
}