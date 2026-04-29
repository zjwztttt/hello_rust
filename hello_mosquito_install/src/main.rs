use std::process::Command;
use std::path::PathBuf;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;
use std::env;
use std::fs;

// 默认配置内容（TOML 格式）
const DEFAULT_SETTING: &str = r#"
# ==============================
# 程序默认配置文件
# ==============================
[general]
# 下载程序支持的网站
supported = [
    "bilibili.com", "iqiyi.com", "qq.com", "youku.com","mgtv.com", "weibo.com", "youtu.be","youtube.com"
]
"#;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取当前程序所在目录
    let dqpath = env::current_exe()?.parent().ok_or("无法获取程序所在目录")?.to_str().ok_or("路径包含非 UTF-8 字符")?.to_string();
    //把当前目录添加到系统级 PATH 环境变量（需要管理员权限）
    // ====================== 1. 打开系统环境变量注册表（需要管理员）
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let env_path = "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment";
    let (env_key, _) = hklm.create_subkey(env_path)?;

    // ====================== 2. 读取当前系统 PATH
    let mut current_path: String = env_key.get_value("Path")?;

    // ====================== 3. 追加新路径（避免重复）
    if !current_path.contains(&dqpath) {
        current_path.push_str(";");
        current_path.push_str(&dqpath);

        // ====================== 4. 写回注册表（setx /m 的核心）
        env_key.set_value("Path", &current_path)?;
        println!("✅ 已将程序当前所在路径添加到系统中");
        println!("⚠️  需重启终端/电脑才能生效");
    } else {
        println!("ℹ️ 程序当前所在路径已存在于系统中");
    }

    //如果当前目录下没有 Download 文件夹，就创建它
    let dir = Path::new("Download");
    if !dir.exists() {
        fs::create_dir(dir).unwrap();
        println!("✅ 已成功创建 Download 文件夹");
    } else {
        println!("ℹ️ Download 文件夹已存在");
    }

    // ==================== 5. 配置文件处理 ====================
    let config_path = PathBuf::from(dqpath.clone()).join("setting.toml");
    //println!("配置文件路径: {}", config_path.display());
    // 3. 检查文件是否存在
    if config_path.exists() {
        println!("✅ 配置文件已就绪！");
    } else {
        // 默认配置内容（TOML 格式）
        let default_config = DEFAULT_SETTING;
        // 创建并写入默认配置
        match fs::write(&config_path, default_config) {
            Ok(_) => println!("✅ 配置文件创建成功！"),
            Err(e) => eprintln!("❌ 创建文件失败: {}", e),
        }
    }

    // 要启动的脚本路径
    let bat_path = format!("{}\\Mosquito.exe", dqpath);
    //注册自定义URL协议
    // ====================== 2. 打开 HKEY_CLASSES_ROOT
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

    // ====================== 3. 注册 F1 协议（逐条对应你的 reg add）
    // reg add HKCR\F1 /f
    let (f1_key, _) = hkcr.create_subkey("F1")?;

    // reg add HKCR\F1 /ve /t Reg_SZ /d "URL:F1 Protocol Handler" /f
    f1_key.set_value("", &"URL:F1 Protocol Handler")?;

    // reg add HKCR\F1 /v "URL Protocol" /t Reg_SZ /d "" /f
    f1_key.set_value("URL Protocol", &"")?;

    // reg add HKCR\F1\shell /f
    let (_shell_key, _) = f1_key.create_subkey("shell")?;

    // reg add HKCR\F1\shell\open /f
    let (open_key, _) = f1_key.create_subkey("shell\\open")?;

    // reg add HKCR\F1\shell\open\command /f
    let (command_key, _) = open_key.create_subkey("command")?;

    // reg add HKCR\F1\shell\open\command /ve /t Reg_SZ /d "xxx\wujinDL.bat" /f
    command_key.set_value("", &bat_path)?;

    // ====================== 完成
    println!("✅ 程序已就绪！");
    println!("👉 现在打开 f1://test 会自动运行 Mosquito.exe");

    //命令提示符 pause
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); 
    Ok(())
}
// 获取当前程序所在目录
//let dqpath = env::current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string();
// 获取当前程序所在目录,完全等价批处理：set "dqpath=%~dp0" + set "dqpath=%dqpath:~0,-1%"
//let dqpath = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
/*获取当前程序所在目录
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 获取当前 exe 文件所在的**完整目录路径**（等价于 %~dp0）
    let mut dqpath = env::current_exe()?; // 得到 exe 完整路径
    dqpath.pop(); // 去掉文件名，只剩下目录（关键）

    // 2. 转成字符串（自动处理 Windows 路径，自动去掉末尾多余的 \）
    let dqpath_str = dqpath.to_string_lossy().replace("\\\\", "\\").to_string();

    println!("{:?}", dqpath);
    println!("当前目录路径：{}", dqpath_str);
    
    Ok(())
}
*/
//如果当前目录下没有 Download 文件夹，就创建它
//std::fs::create_dir_all("Download").unwrap();
/*
fn main() -> std::io::Result<()> {
    // 👇 这一行 = 批处理的 if not exist Download md ".\Download"
    fs::create_dir_all("Download")?;

    println!("Download 文件夹已准备就绪");
    Ok(())
}
*/

/*把当前目录添加到系统级 PATH 环境变量（需要管理员权限）
use std::env;
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> std::io::Result<()> {
    // ====================== 1. 获取当前 exe 所在目录（等价 %~dp0）
    let exe_path = env::current_exe()?;
    let dqpath = exe_path
        .parent()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "获取目录失败"))?
        .to_str()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "路径无效"))?;

    println!("要添加到 PATH 的路径：{}", dqpath);

    // ====================== 2. 打开系统环境变量注册表（需要管理员）
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let env_path = "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment";
    let (env_key, _) = hklm.create_subkey(env_path)?;

    // ====================== 3. 读取当前系统 PATH
    let mut current_path: String = env_key.get_value("Path")?;

    // ====================== 4. 追加新路径（避免重复）
    if !current_path.contains(dqpath) {
        current_path.push_str(";");
        current_path.push_str(dqpath);

        // ====================== 5. 写回注册表（setx /m 的核心）
        env_key.set_value("Path", &current_path)?;
        println!("✅ 成功添加到 系统 PATH！");
        println!("⚠️  需重启终端/电脑才能生效");
    } else {
        println!("ℹ️ 路径已存在于 PATH 中");
    }

    Ok(())
}
*/