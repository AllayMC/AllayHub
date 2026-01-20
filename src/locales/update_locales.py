#!/usr/bin/env python3
"""
Helper script for updating locale files in AllayHub.
Usage:
    python update_locales.py <command> [options]

Commands:
    replace <key> <old_text> <new_text>  - Replace text in a specific key across all locales
    set <key> <locale> <new_text>        - Set text for a specific key in a specific locale
    find <pattern>                        - Find keys matching a pattern
    validate                              - Validate all JSON files
    show <key>                            - Show a key's value in all locales
"""

import json
import os
import sys
import re
from pathlib import Path

LOCALES_DIR = Path(__file__).parent


def get_locale_files():
    """Get all locale index.json files."""
    locales = {}
    for item in LOCALES_DIR.iterdir():
        if item.is_dir():
            index_file = item / "index.json"
            if index_file.exists():
                locales[item.name] = index_file
    return locales


def load_locale(filepath):
    """Load a locale JSON file."""
    with open(filepath, 'r', encoding='utf-8') as f:
        return json.load(f)


def save_locale(filepath, data):
    """Save a locale JSON file with proper formatting."""
    with open(filepath, 'w', encoding='utf-8') as f:
        json.dump(data, f, ensure_ascii=False, indent=2)
    print(f"  Saved: {filepath}")


def validate_all():
    """Validate all locale JSON files."""
    locales = get_locale_files()
    errors = []
    for locale, filepath in sorted(locales.items()):
        try:
            load_locale(filepath)
            print(f"  OK: {locale}")
        except json.JSONDecodeError as e:
            errors.append((locale, str(e)))
            print(f"  ERROR: {locale} - {e}")

    if errors:
        print(f"\n{len(errors)} file(s) with errors")
        return False
    print(f"\nAll {len(locales)} locale files are valid!")
    return True


def show_key(key):
    """Show a key's value in all locales."""
    locales = get_locale_files()
    for locale, filepath in sorted(locales.items()):
        try:
            data = load_locale(filepath)
            if key in data:
                msg = data[key].get("message", "")
                print(f"  {locale}: {msg}")
            else:
                print(f"  {locale}: <not found>")
        except Exception as e:
            print(f"  {locale}: <error: {e}>")


def find_keys(pattern):
    """Find keys matching a pattern in any locale."""
    locales = get_locale_files()
    found_keys = set()
    regex = re.compile(pattern, re.IGNORECASE)

    for locale, filepath in locales.items():
        try:
            data = load_locale(filepath)
            for key in data.keys():
                if regex.search(key):
                    found_keys.add(key)
        except Exception:
            pass

    for key in sorted(found_keys):
        print(f"  {key}")
    print(f"\nFound {len(found_keys)} matching key(s)")


def replace_in_key(key, old_text, new_text):
    """Replace text in a specific key across all locales."""
    locales = get_locale_files()
    updated = 0

    for locale, filepath in sorted(locales.items()):
        try:
            data = load_locale(filepath)
            if key in data:
                msg = data[key].get("message", "")
                if old_text in msg:
                    data[key]["message"] = msg.replace(old_text, new_text)
                    save_locale(filepath, data)
                    updated += 1
                    print(f"  Updated: {locale}")
        except Exception as e:
            print(f"  Error in {locale}: {e}")

    print(f"\nUpdated {updated} file(s)")


def set_key_value(key, locale, new_text):
    """Set a specific key's value in a specific locale."""
    locales = get_locale_files()

    if locale not in locales:
        print(f"Locale '{locale}' not found")
        return

    filepath = locales[locale]
    try:
        data = load_locale(filepath)
        if key not in data:
            data[key] = {}
        data[key]["message"] = new_text
        save_locale(filepath, data)
        print(f"Set {key} in {locale}")
    except Exception as e:
        print(f"Error: {e}")


def bulk_update(key, updates):
    """
    Bulk update a key across multiple locales.
    updates: dict of {locale: new_message}
    """
    locales = get_locale_files()

    for locale, new_text in updates.items():
        if locale not in locales:
            print(f"  Skipped: {locale} (not found)")
            continue

        filepath = locales[locale]
        try:
            data = load_locale(filepath)
            if key not in data:
                data[key] = {}
            data[key]["message"] = new_text
            save_locale(filepath, data)
        except Exception as e:
            print(f"  Error in {locale}: {e}")


def replace_all_occurrences(old_text, new_text):
    """Replace text in all keys across all locales."""
    locales = get_locale_files()
    total_updated = 0

    for locale, filepath in sorted(locales.items()):
        try:
            data = load_locale(filepath)
            updated = False
            for key in data:
                if "message" in data[key]:
                    msg = data[key]["message"]
                    if old_text in msg:
                        data[key]["message"] = msg.replace(old_text, new_text)
                        updated = True

            if updated:
                save_locale(filepath, data)
                total_updated += 1
        except Exception as e:
            print(f"  Error in {locale}: {e}")

    print(f"\nUpdated {total_updated} file(s)")


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return

    command = sys.argv[1]

    if command == "validate":
        validate_all()

    elif command == "show" and len(sys.argv) >= 3:
        key = sys.argv[2]
        print(f"Key: {key}")
        show_key(key)

    elif command == "find" and len(sys.argv) >= 3:
        pattern = sys.argv[2]
        print(f"Finding keys matching: {pattern}")
        find_keys(pattern)

    elif command == "replace" and len(sys.argv) >= 5:
        key = sys.argv[2]
        old_text = sys.argv[3]
        new_text = sys.argv[4]
        print(f"Replacing '{old_text}' with '{new_text}' in key '{key}'")
        replace_in_key(key, old_text, new_text)

    elif command == "replace-all" and len(sys.argv) >= 4:
        old_text = sys.argv[2]
        new_text = sys.argv[3]
        print(f"Replacing '{old_text}' with '{new_text}' in ALL keys")
        replace_all_occurrences(old_text, new_text)

    elif command == "set" and len(sys.argv) >= 5:
        key = sys.argv[2]
        locale = sys.argv[3]
        new_text = sys.argv[4]
        set_key_value(key, locale, new_text)

    else:
        print(__doc__)


if __name__ == "__main__":
    main()
