#!/usr/bin/env python3.9

from typing import List
import re

LANGUAGES = ["de", "en", "es", "fr", "pt"]
IN_FILE_PREFIX = "hermitdave_frequencywords"
IN_FILE_SUFFIX = "50k.txt"
OUT_FILE_PREFIX = "raw_dict"
OUT_FILE_SUFFIX = ".txt"

MIN_WORD_LENGTH = 4
WHITELISTED_PATTERN_EN = r"[^a-zA-Z]"
WHITELISTED_PATTERN_OTHERS = r"[^a-zA-Zà-úÀ-Úä-üÄ-Üàáâãäåèéêëìíîïòóôõöùúûüßæçñœ]"  # noqa: E501


def read_file(lang: str) -> List[str]:
    def _pre_filter(line: str) -> bool:
        is_comment = line.startswith("#")
        too_short = len(line.split(' ')[0]) < MIN_WORD_LENGTH
        return not is_comment and not too_short

    def _post_filter(line: str) -> bool:
        if lang == "en":
            pattern = WHITELISTED_PATTERN_EN
        else:
            pattern = WHITELISTED_PATTERN_OTHERS

        return re.search(pattern, line) is None

    with open(f"{IN_FILE_PREFIX}_{lang}_{IN_FILE_SUFFIX}", "r") as file:
        print(f"\nReading input file {file.name}")
        lines = file.readlines()

        pre_filtered = filter(_pre_filter, lines)
        lines = [line.split(" ")[0] for line in pre_filtered]
        print(f"Read {len(lines)} lines")

        words = list(filter(_post_filter, lines[0:15000]))
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
