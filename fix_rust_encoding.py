# -*- coding: utf-8 -*-
"""
修复 Rust 文件的编码问题
将所有文件从可能的 GBK/CP1252 编码转换为 UTF-8
"""
import os
import sys

def fix_file_encoding(file_path):
    """修复单个文件的编码"""
    try:
        # 读取文件原始字节
        with open(file_path, 'rb') as f:
            raw_bytes = f.read()
        
        # 尝试检测编码
        encodings_to_try = ['gbk', 'gb2312', 'cp1252', 'utf-8']
        content = None
        detected_encoding = None
        
        for encoding in encodings_to_try:
            try:
                content = raw_bytes.decode(encoding)
                detected_encoding = encoding
                break
            except UnicodeDecodeError:
                continue
        
        if content is None:
            # 如果都失败，使用 utf-8 并替换错误字符
            content = raw_bytes.decode('utf-8', errors='replace')
            detected_encoding = 'utf-8 (with replacements)'
        
        # 重新写入 UTF-8 编码（无BOM）
        with open(file_path, 'wb') as f:
            f.write(content.encode('utf-8'))
        
        return True, detected_encoding
    except Exception as e:
        return False, str(e)

def main():
    # 获取要修复的文件列表
    base_dir = 'src-tauri/src/shared/http_api/lcu'
    
    if not os.path.exists(base_dir):
        print(f"Directory not found: {base_dir}")
        return
    
    # 查找所有 .rs 文件
    rs_files = []
    for root, dirs, files in os.walk(base_dir):
        for file in files:
            if file.endswith('.rs'):
                rs_files.append(os.path.join(root, file))
    
    print(f"Found {len(rs_files)} Rust files to check")
    
    fixed_count = 0
    for file_path in rs_files:
        success, info = fix_file_encoding(file_path)
        if success:
            if info != 'utf-8':
                print(f"Fixed: {file_path} (was {info})")
                fixed_count += 1
        else:
            print(f"Error fixing {file_path}: {info}")
    
    print(f"\nFixed {fixed_count} files")
    print("Done!")

if __name__ == '__main__':
    main()

