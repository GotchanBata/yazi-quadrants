#!/usr/bin/env bash
set -e

echo -e "\033[1;34m[1/4] Проверка зависимостей...\033[0m"
if ! command -v cargo &> /dev/null; then
    echo -e "\033[0;31mОшибка: Rust (cargo) не установлен. Установите его с https://rustup.rs/\033[0m"
    exit 1
fi
if ! command -v ffmpeg &> /dev/null; then
    echo -e "\033[0;31mОшибка: ffmpeg не установлен. Пожалуйста, установите его через ваш пакетный менеджер.\033[0m"
    exit 1
fi

echo -e "\033[1;34m[2/4] Компиляция yazi-quadrants...\033[0m"
TMP_DIR=$(mktemp -d)
git clone https://github.com/georg/yazi-quadrants.git "$TMP_DIR"
cd "$TMP_DIR"
cargo build --release

echo -e "\033[1;34m[3/4] Установка бинарного файла...\033[0m"
mkdir -p ~/.local/bin
cp target/release/yazi-quadrants ~/.local/bin/
chmod +x ~/.local/bin/yazi-quadrants

echo -e "\033[1;34m[4/4] Установка плагина Yazi...\033[0m"
PLUGIN_DIR="$HOME/.config/yazi/plugins/video-quadrants.yazi"
mkdir -p "$PLUGIN_DIR"
cp yazi-plugin/main.lua "$PLUGIN_DIR/"

rm -rf "$TMP_DIR"

echo -e "\033[1;32mУстановка успешно завершена!\033[0m"
echo -e "\033[1;33mОстался один шаг. Добавьте следующие строки в ваш ~/.config/yazi/yazi.toml:\033[0m"
echo ""
echo -e "  \033[0;36m[plugin]\033[0m"
echo -e "  \033[0;36mprepend_previewers = [\033[0m"
echo -e "      \033[0;36m{ mime = \"video/*\", run = \"video-quadrants\" }\033[0m"
echo -e "  \033[0;36m]\033[0m"
echo ""
echo -e "\033[1;33m(Также убедитесь, что ~/.local/bin добавлена в ваш \$PATH)\033[0m"
