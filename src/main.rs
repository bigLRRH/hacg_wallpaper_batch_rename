use std::{error::Error, fs};

use regex::Regex;

const SOURCE_PATH: &str = r"D:\Resources\pictures\wallpaper_cache";
const TARGET_PATH: &str = r"D:\Resources\pictures\wallpapers";

fn main() -> Result<(), Box<dyn Error>> {
    let source_path = std::path::Path::new(SOURCE_PATH);
    let target_path = std::path::Path::new(TARGET_PATH);

    // 创建目标目录（如果不存在）
    if !target_path.exists() {
        fs::create_dir_all(target_path)?;
    }

    // 匹配格式：琉璃神社壁纸包 + 年月
    let dir_re = Regex::new(r"琉璃神社壁纸包 (\d{4})年(0?[1-9]|1[0-2])月号")?;
    let mut matched_dirs = Vec::new();

    // 遍历源目录
    for entry in fs::read_dir(source_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_string_lossy();
            if dir_re.is_match(&dir_name) {
                matched_dirs.push(path.clone());
                println!("Matched: {}", dir_name);
            }
        }
    }

    // 处理匹配到的目录
    for dir_path in matched_dirs {
        // 提取年月信息
        let dir_name = dir_path.file_name().unwrap().to_string_lossy();
        let captures = dir_re.captures(&dir_name).unwrap();
        let (year, month) = (captures[1].parse::<u16>()?, captures[2].parse::<u8>()?);
        // 格式化为6位年月字符串（YYYYMM）
        let year_month = format!("{:04}{:02}", year, month);

        println!("Processing: {}", dir_name);
        println!("year_month: {}", year_month);

        // todo 目标文件改名序号前补0
        // 遍历目录中的文件
        for entry in fs::read_dir(&dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let file_name = path.file_name().unwrap().to_string_lossy();
                println!("Copying: {}", file_name);
                let target_file_name = format!("{}{}", year_month, file_name);
                println!("To: {}", target_file_name);
                let target_file_path = target_path.join(target_file_name);
                println!("target_file_path: {:?}", target_file_path);
                // fs::copy(&path, &target_file_path)?;
            }
        }
    }

    Ok(())
}
