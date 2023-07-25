from colorama import Fore, Style
import re

default_algorithm = "- 0 1 1 2 0.4"

common_words = [
    "the",
    "be",
    "to",
    "of",
    "and",
    "a",
    "an",
    "it",
    "at",
    "on",
    "he",
    "she",
    "but",
    "is",
    "my",
]

def parse_algorithm(algorithm):
    try:
        res = {
            "exclude": True,
            "sizes": [],
            "restRatio": 0.4,
        }
        parts = algorithm.split(" ")

        if parts[0] == "+":
            res["exclude"] = False

        res["restRatio"] = float(parts[-1])

        for i in range(1, len(parts) - 1):
            res["sizes"].append(int(parts[i]))

        return res
    except Exception as e:
        print("not parsed")
        return {
            "exclude": True,
            "sizes": [1, 1, 2],
            "restRatio": 0.4,
        }

algorithm = parse_algorithm(default_algorithm)

def bionify_word(word):
    if word.lower() in common_words or len(word) <= 3:
        return f'{Style.BRIGHT}{Fore.GREEN}{word}{Style.RESET_ALL}'

    num_bold = 1
    index = len(word) - 1

    if index < len(algorithm['sizes']):
        num_bold = algorithm['sizes'][index]
    else:
        num_bold = index

    return f'{Style.BRIGHT}{Fore.GREEN}{word[:num_bold]}{Style.RESET_ALL}{word[num_bold:]}'

def bionify_text(text):
    if len(text) < 10:
        return text

    words = text.split(" ")
    res = " ".join([bionify_word(word) for word in words])
    return res

test_string = (
    "Welcome to the fast lane, my friend. See, by selectrively drawing your attetion to"
    "certain parts of the sentence we create a gap that our psychology is programmed to fil. So, we"
    " need less and can run times faster."
)

print(bionify_text(test_string))
