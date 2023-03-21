#!/usr/bin/env python3.9

from typing import Dict, Iterable, Set
import re

LANGUAGES = ["de", "en", "es", "fr", "pt"]
MIN_WORD_LENGTH = 4
MAX_WORD_LENGTH = 10
MAX_WORDS_PER_LENGTH = 1500


def read_dict_file(lang: str) -> Iterable[str]:
    file_name = f"raw_dict_{lang}.txt"

    with open(file_name, "r") as file:
        lines = file.readlines()
        print(f"Read {len(lines)} lines from {file_name}")
        return filter(lambda line: not line.startswith("#"), lines)


def remove_accents(original: str) -> str:
    word = original.strip().lower()

    # Accents
    word = re.sub(r"[àáâãäå]", "a", word)
    word = re.sub(r"[èéêë]", "e", word)
    word = re.sub(r"[ìíîï]", "i", word)
    word = re.sub(r"[òóôõö]", "o", word)
    word = re.sub(r"[ùúûü]", "u", word)

    # Special characters
    word = re.sub(r"[ß]", "ss", word)
    word = re.sub(r"[æ]", "ae", word)
    word = re.sub(r"[ç]", "c", word)
    word = re.sub(r"[ñ]", "n", word)
    word = re.sub(r"[œ]", "ce", word)

    return word


def group_words_by_length(all_words: Iterable[str]) -> Dict[int, Set[str]]:
    grouped: Dict[int, Set[str]] = dict()

    for line in all_words:
        word = remove_accents(line)
        length = len(word)

        if length < MIN_WORD_LENGTH or length > MAX_WORD_LENGTH:
            continue

        if length not in grouped:
            grouped[length] = set()

        if len(grouped[length]) >= MAX_WORDS_PER_LENGTH:
            continue

        grouped[length].add(word)

    return grouped


if __name__ == "__main__":
    for lang in LANGUAGES:
        print(f"\nReading raw file for '{lang}'")

        all_words = read_dict_file(lang)
        grouped_words = group_words_by_length(all_words)

        with open(f"dict_{lang}.txt", "w") as file:
            for length, words in grouped_words.items():
                print(f"Writing {len(words)} words with length of {length}")
                file.write(f"{length}:{','.join(words)}\n")
