# hacg_wallpaper_batch_rename

批量重命名琉璃神社壁纸  

批量重命名 source 路径下的琉璃神社壁纸，并保存到 target 路径，格式为 "{year_month}{num:03}"，保留原扩展名。

source 目录包含按年月命名的文件夹，如 琉璃神社壁纸包 2017年8月号，其中图片命名为两位数（如 01、02）。
target 目录仅包含格式化文件，如 201708001、201708002，其中 201708 代表年月，001 为原文件的编号，不足三位补零，扩展名不变。

能够自动识别最多最长的前缀

## todo list
- [ ] 加用户接口，优化代码，找bug

## bug list
- [ ] 同一个文件夹可能不能执行两次程序