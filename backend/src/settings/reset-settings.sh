cd ~/Run/language_generator/backend/src/settings/

for file in *.bak; do
    new_name="${file%.bak}.json"
    mv "$file" "$new_name"
done
