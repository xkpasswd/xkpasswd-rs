#!/usr/bin/env python3.9

from typing import List
import re

LANGUAGES = ["de", "en", "es", "fr", "pt"]
IN_FILE_PREFIX = "hermitdave_frequencywords"
IN_FILE_SUFFIX = "50k.txt"
OUT_FILE_PREFIX = "raw_dict"
OUT_FILE_SUFFIX = ".txt"


def read_file(lang: str) -> List[str]:
    def _line_filter(line: str) -> bool:
        is_comment = line.startswith("#")
        should_ignore = re.search(r"[\.\'\-\d]", line) is not None
        return not is_comment and not should_ignore

    with open(f"{IN_FILE_PREFIX}_{lang}_{IN_FILE_SUFFIX}", "r") as file:
        print(f"\nReading input file {file.name}")
        lines = file.readlines()
        print(f"Read {len(lines)} lines")

        sanitized_lines = [line.split(" ")[0] for line in lines]
        words = list(filter(_line_filter, sanitized_lines))
        words.sort()
        return words


def write_file(lang: str, words: List[str]):
    with open(f"{OUT_FILE_PREFIX}_{lang}{OUT_FILE_SUFFIX}", "w") as file:
        print(f"Writing {len(words)} words to output file {file.name}")
        file.writelines("\n".join(words))


if __name__ == "__main__":
    for lang in LANGUAGES:
        words = read_file(lang)
        write_file(lang, words)
