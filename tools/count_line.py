import os

# カレントディレクトリ以下の指定したファイルタイプの行数をカウントする
# 除外するディレクトリを指定できる
# 検索したファイルのリストをline.txtに出力し、各ファイルの行数をカウント

# 除外するディレクトリ
EXCLUDE_DIRS = {'.venv', 'static', 'migrations', 'staticfiles', 'node_modules', 'dist', 'target'}

# 検索するファイルタイプとその拡張子
FILE_TYPES = {
    'Rust': ['.rs'],  # Rustのファイル拡張子
    'Rust Test': ['.rs'],  # テスト用のRustファイル（判定を別途実装）
}

def count_lines(file_path):
    """ファイルの行数をカウントする"""
    with open(file_path, 'r', encoding='utf-8') as f:
        return sum(1 for _ in f)

def is_rust_test_file(file_path):
    """Rustのテストファイルかどうかを判定する"""
    return any(keyword in file_path for keyword in ['_test', 'mod.rs'])

def main():
    total_lines = 0
    lines_by_type = {ft: 0 for ft in FILE_TYPES}

    with open('line.txt', 'w') as file_out:
        for root, dirs, files in os.walk('./'):
            # 除外するディレクトリをスキップ
            dirs[:] = [d for d in dirs if d not in EXCLUDE_DIRS]

            for file in files:
                for file_type, extensions in FILE_TYPES.items():
                    if any(file.endswith(ext) for ext in extensions):
                        file_path = os.path.join(root, file)
                        lines = count_lines(file_path)

                        file_out.write(f"{file_path}: {lines}\n")

                        if file_type == 'Rust':
                            if is_rust_test_file(file_path):
                                lines_by_type['Rust Test'] += lines
                            else:
                                lines_by_type['Rust'] += lines
                        else:
                            lines_by_type[file_type] += lines

                        total_lines += lines
                        break

    # 結果を表示
    for file_type, lines in lines_by_type.items():
        print(f"{file_type} files: {lines} lines")
    print(f"Total lines: {total_lines}")

if __name__ == '__main__':
    main()
