import re
from colorama import Fore, Style

def is_vowel(char, char_index, word):
    vowels = ["a", "e", "i", "o", "u"]
    if char.lower() == "y":
        if char_index == 0 or (char_index < len(word) - 1 and word[char_index + 1].lower() in vowels):
            return False
        return True
    return char.lower() in vowels

def highlight_on_first_syllable(word):
    vowel_clusters = ["au", "ai", "ea", "ee", "ei", "eu", "ie", "io", "oa", "oe", "oi", "oo", "ou", "ue", "ui"]
    coda_exceptions = ["gh", "nd", "ld", "st"]
    
    if len(word) <= 1:
        return 1
    if len(word) <= 4:
        return len(word) // 2

    for cur_char_index in range(len(word)):
        if cur_char_index < len(word) - 1:
            substring = word[cur_char_index:cur_char_index + 2].lower()
            if substring in vowel_clusters:
                return cur_char_index + 2

        coda = 1
        char = word[cur_char_index]
        if is_vowel(char, cur_char_index, word):
            if cur_char_index + coda < len(word):
                next_chars = word[cur_char_index + coda:cur_char_index + coda + 2].lower()
                if next_chars in coda_exceptions:
                    coda += 1
            return cur_char_index + coda

    return 1

def bionify_word(word):
    num_bold = highlight_on_first_syllable(word)
    return f'{Style.BRIGHT}{Fore.GREEN}{word[:num_bold]}{Style.RESET_ALL}{word[num_bold:]}'

def bionify_text(text):
    if len(text) < 10:
        return text

    words = re.findall(r'\b\w+\b', text)
    res = " ".join([bionify_word(word) for word in words])
    return res

test_string = (
    "Hey there! What is up? Welcome to the fast lane, my friend. See, by selectrively drawing your attetion to"
    "certain parts of the sentence we create a gap that our psychology is programmed to fil. So, we"
    " need less and can run times faster."
)

print(bionify_text(test_string))
