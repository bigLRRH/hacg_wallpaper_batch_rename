mod trie;

use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use ansi_term::Colour::{Blue, Green};
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
        // 1.提取文件前缀
        let mut trie = trie::Trie::new();

        // 遍历目录中的文件, 生成前缀树
        for entry in fs::read_dir(&dir_path.path_buf)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let file_stem = path.file_stem().unwrap().to_string_lossy().to_string();
            trie.insert(file_stem);

            println!("{:?}", path);
        }

        let mut common_prefix = trie.generalized_longest_common_prefix(2);
        // 移除common_prefix后面的数字部分
        let re = Regex::new(r"\d+$").unwrap();
        common_prefix = re.replace_all(&common_prefix, "").to_string();
        println!("common_prefix: {}", common_prefix);

        // 2.生成新文件名
        for entry in fs::read_dir(&dir_path.path_buf)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            // 如果文件有common_prefix, 则重命名
            let file_stem = path.file_stem().unwrap().to_string_lossy();
            println!("file_stem: {}", file_stem);
            if file_stem.starts_with(&common_prefix) {
                // 提取数字部分,并补齐到3位
                let re = Regex::new(r"\d+$").unwrap();
                match re.find(&file_stem) {
                    None => {
                        println!(
                            "{}{}",
                            Blue.paint("No number found in file_stem: "),
                            file_stem
                        );
                        continue;
                    }
                    Some(num) => {
                        let num = num.as_str().parse::<u16>().unwrap();
                        let num_str = format!("{:03}", num);
                        println!("num_str: {}", num_str);
                        // 生成新文件名
                        let new_file_name = format!(
                            "{}{}.{}",
                            dir_path.year_month,
                            num_str,
                            path.extension().unwrap().to_string_lossy()
                        );
                        println!("new_file_name: {}", new_file_name);
                        let new_file_path = target.join(new_file_name);
                        println!("new_file_path: {:?}", new_file_path);
                        // 移动文件
                        println!(
                            "{}{:?} -> {:?}",
                            Green.paint("Moving file: "),
                            path,
                            new_file_path
                        );
                        fs::rename(&path, new_file_path)?;
                    }
                }
            }
        }
    }
    Ok(())
}
