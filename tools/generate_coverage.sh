#!/bin/bash

# プロジェクトルートに移動
cd "$(dirname "$0")/.."

# 古いカバレッジデータを削除
rm -f *.profraw

echo "Running tests with coverage instrumentation..."
RUSTFLAGS="-C instrument-coverage" cargo test --workspace

echo "Generating coverage report..."
mkdir -p coverage
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing --ignore "/*" -o ./coverage/
grcov . -s . --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing --ignore "/*" -o ./coverage/lcov.info

# 中間ファイルを削除
rm *.profraw

echo "Coverage report generated at coverage/html/index.html"
echo "LCOV data generated at coverage/lcov.info"
