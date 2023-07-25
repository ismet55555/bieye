from colorama import Fore, Style

def bionic_reading(text):
    """
    This function applies the Bionic Reading algorithm to a given text.
    It simply bolds the first 1-3 characters of each word.

    Args:
        text: The text to be processed.

    Returns:
        The text with the bolded letters.
    """
    # Split the text into words.
    words = text.split()

    bionic_text = ""

    # Iterate through each word in the text
    for word in words:
        # The first 1-3 characters of each word are made bold.
        if len(word) > 3:
            bionic_text += Fore.GREEN + Style.BRIGHT + word[:3] + Style.RESET_ALL + word[3:] + " "
        else:
            bionic_text += Fore.GREEN + Style.BRIGHT + word + Style.RESET_ALL + " "

    return bionic_text


def main():
    # Get the text to be processed.
    text = input("Enter the text to be processed: ")

    # Apply the Bionic Reading algorithm.
    bionic_text = bionic_reading(text)

    # Print the bionic text.
    print(bionic_text)


if __name__ == "__main__":
    main()
