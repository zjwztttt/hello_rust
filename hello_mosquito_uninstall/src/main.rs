use std::process::Command;
use winreg::{enums::HKEY_CLASSES_ROOT,RegKey};
use std::io;

/// 卸载 Windows 注册表中的 F1 协议处理
fn uninstall_f1_protocol() -> io::Result<()> {
    // 打开 HKEY_CLASSES_ROOT 根键
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

    // 尝试删除（不存在时不视为错误）
    match hkcr.delete_subkey_all("F1") {
        Ok(_) => println!("✅ 成功卸载 F1 协议"),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            println!("ℹ️ F1 协议不存在，无需卸载");
        }
        Err(e) => {
            eprintln!("❌ 卸载失败：{}", e);
            return Err(e);
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("正在卸载 F1 协议...\n");
    uninstall_f1_protocol()?;
    //命令提示符 pause
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); 
    Ok(())
}
