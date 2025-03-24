use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use regex::Regex;

const SOURCE_PATH: &str = r"D:\Resources\pictures\wallpaper_cache";
const TARGET_PATH: &str = r"D:\Resources\pictures\wallpapers";

struct DirInfo {
    path_buf: PathBuf,
    year_month: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let source = std::path::Path::new(SOURCE_PATH);
    let target = std::path::Path::new(TARGET_PATH);

    // 创建目标目录（如果不存在）
    if !target.exists() {
        fs::create_dir_all(target)?;
    }

    // 匹配格式：琉璃神社壁纸包 + 年月
    let dir_re = Regex::new(r"琉璃神社壁纸包 (\d{4})年(0?[1-9]|1[0-2])月号")?;

    process_matched_dirs(match_dirs(source, &dir_re)?, target)?;

    Ok(())
}

fn match_dirs(source: &Path, re: &Regex) -> Result<Vec<DirInfo>, Box<dyn Error>> {
    let mut matched_dirs = Vec::new();

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_string_lossy();
            if re.is_match(&dir_name) {
                let captures = re.captures(&dir_name).unwrap();
                let (year, month) = (captures[1].parse::<u16>()?, captures[2].parse::<u8>()?);
                let year_month = format!("{:04}{:02}", year, month);
                matched_dirs.push(DirInfo {
                    path_buf: path,
                    year_month,
                });
            }
        }
    }

    Ok(matched_dirs)
}

fn process_matched_dirs(matched_dirs: Vec<DirInfo>, target: &Path) -> Result<(), Box<dyn Error>> {
    for dir_path in matched_dirs {
        // 遍历目录中的文件
        for entry in fs::read_dir(&dir_path.path_buf)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            // todo 提取文件前缀和编号，生成新文件名
        }
    }
    Ok(())
}
