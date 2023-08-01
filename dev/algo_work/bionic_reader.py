import string

threshold = 1

letter_frequency = {
  "a": 8.167,
  "b": 1.506,
  "c": 2.782,
  "d": 4.253,
  "e": 12.702,
  "f": 2.228,
  "g": 2.015,
  "h": 6.094,
  "i": 7.546,
  "j": 0.153,
  "k": 0.772,
  "l": 4.025,
  "m": 2.406,
  "n": 6.749,
  "o": 7.507,
  "p": 1.929,
  "q": 0.095,
  "r": 6.094,
  "s": 6.327,
  "t": 9.056,
  "u": 2.802,
  "v": 0.978,
  "w": 2.36,
  "x": 0.15,
  "y": 1.974,
  "z": 0.074,
}

part_of_speech = {
  "a": "adjective",
  "b": "noun",
  "c": "verb",
  "d": "adverb",
  "e": "pronoun",
  "f": "conjunction",
  "g": "preposition",
  "h": "interjection",
  "i": "article",
  "j": "exclamation",
  "k": "number",
  "l": "symbol",
  "m": "other",
  "n": "punctuation",
  "o": "none",
  "p": "foreign",
  "q": "abbreviation",
  "r": "unknown",
  "s": "special",
  "t": "ellipsis",
  "u": "contraction",
  "v": "dash",
  "w": "hyphenated",
  "x": "slash",
  "y": "apostrophe",
  "z": "quote",
}

def position(letter, word):
  """
  This function returns the position of the given letter in the given word.

  Args:
    letter: The letter to be found.
    word: The word in which to find the letter.

  Returns:
    The position of the letter in the word.
  """

  for i, char in enumerate(word):
    if char == letter:
      return i
  return -1


def bionic_reading(text):
  """
  This function applies the Bionic Reading algorithm to a given text.

  Args:
    text: The text to be processed.

  Returns:
    The text with the bolded letters.
  """

  # Split the text into words.
  words = text.split()

  # Create a list of scores for each letter in each word.
  scores = []
  for word in words:
    for letter in word:
      score = 0
      # Add the frequency of the letter to the score.
      score += letter_frequency[letter]
      # Add the position of the letter to the score.
      score += (len(word) - position(letter, word))
      # Add the part of speech of the word to the score.
      # score += part_of_speech[word]
      scores.append(score)

  # Find the letters with the highest scores.
  high_score_letters = []
  for score in scores:
    if score > threshold:
      high_score_letters.append(letter)

  # Bold the high score letters.
  bolded_text = ""
  for word in words:
    bolded_text += "".join(high_score_letters) + " "

  return bolded_text


def main():
  # Get the text to be processed.
  text = input("Enter the text to be processed: ")

  # Apply the Bionic Reading algorithm.
  bolded_text = bionic_reading(text)

  # Print the bolded text.
  print(bolded_text)


if __name__ == "__main__":
  main()

