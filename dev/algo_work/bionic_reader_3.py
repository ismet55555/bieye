from nltk.corpus import brown
import nltk
import string
from colorama import Fore, Style

# We will use the Brown corpus for frequency analysis
fdist = nltk.FreqDist(w.lower() for w in brown.words())

def bionic_reading(text):
    words = nltk.word_tokenize(text)
    processed_words = []

    for word in words:
        if len(word) > 3 and word not in string.punctuation:
            word_pos = nltk.pos_tag([word])[0][1]
            # for smaller words, we only bold the first letter
            # for larger words, we bold the first, second and last letter
            # common words or words of significant POS (Noun, Verb, Adjective) we bold the entire word
            if word.lower() in fdist and fdist[word.lower()] > 3000 or word_pos in ['NN', 'NNS', 'NNP', 'NNPS', 'VB', 'VBD', 'VBG', 'VBN', 'VBP', 'VBZ', 'JJ']:
                processed_word = Fore.GREEN + Style.BRIGHT + word + Style.RESET_ALL
            else:
                processed_word = Fore.GREEN + Style.BRIGHT + word[:3] + Style.RESET_ALL + word[3:]
        else:
            processed_word = word
        processed_words.append(processed_word)
    return processed_words

def main():
    text = input("Enter the text to be processed: ")
    print(' '.join(bionic_reading(text)))

if __name__ == "__main__":
    nltk.download('punkt')
    nltk.download('brown')
    nltk.download('averaged_perceptron_tagger')
    main()
