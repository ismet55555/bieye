from colorama import Style, Fore
import re

col = False
dim = True


def parse_text(text):
    # words = re.findall(r'\b\w+\b', text)
    
    # Regex for words including punctuation adjacent to it
    # words = re.findall(r'\w+|[^\w\s]', text)

    words = re.findall(r'\w+|[^\w]|[\s]', text)


    for index in range(len(words)):
        print(words[index], end=' -> ')

        # Punctuation and symbols
        if re.match(r'^[\W_]+$', words[index]):
            words[index] = (
                f'{Style.BRIGHT}{Fore.RED if col else ""}'
                f'{words[index]}'
                f'{Style.RESET_ALL}'
            )
            print(words[index])
            continue

        # Numbers
        try:
            int(words[index])
            words[index] = (
                f'{Style.BRIGHT}{Fore.GREEN if col else ""}'
                f'{words[index]}'
                f'{Style.RESET_ALL}'
            )
            print(words[index])
            continue
        except ValueError:
            # Is text, not number
            pass

        # Three letter words
        if len(words[index]) == 3:
            first_half = words[index][:2]
            second_half = words[index][2:]
            words[index] = (
                f'{Style.BRIGHT}{Fore.CYAN if col else ""}'
                f'{first_half}'
                f'{Style.RESET_ALL}'
                f'{second_half}'
            )
            print(words[index])
            continue

        # Very short words
        if len(words[index]) < 3:
            words[index] = (
                f'{Style.BRIGHT}{Fore.BLUE if col else ""}'
                f'{words[index]}'
                f'{Style.RESET_ALL}'
            )
            print(words[index])
            continue

        # Anything else
        ptr = len(words[index]) // 2
        first_half = words[index][:ptr]
        second_half = words[index][ptr:]
        words[index] = (
            f'{Style.BRIGHT}{Fore.YELLOW if col else ""}'
            f'{first_half}'
            f'{Style.RESET_ALL}'
            f'{Style.DIM if dim else ""}'
            f'{second_half}'
            f'{Style.RESET_ALL}'
        )
        print(words[index])

    return ''.join(words)

# test_string = "un-believable sub-command has alot did too"

# test_string = (
#     "Hey there! What is up? Welcome 34 235523 to the fast lane, my friend. See, by selectrively drawing your attetion to"
#     "certain parts of the sentence we create a gap that our psychology is programmed to fill. So, we"
#     " need less and can run times faster."
# )
#
test_string = """
nmcli
A command-line tool for controlling NetworkManager.Some subcommands such as nmcli monitor have their own usage documentation.More information: https://networkmanager.dev/docs/api/latest/nmcli.html.

 - Run an nmcli subcommand:
   nmcli {{agent|connection|device|general|help|monitor|networking|radio}} {{command_options}}

 - Display the current version of NetworkManager:
   nmcli --version

 - Display help:
   nmcli --help

 - Display help for a subcommand:
   nmcli {{subcommand}} --help

Hey there! What is up? Pi is 3.23425 Welcome 34 235523 to the fast lane, my friend. See, by selectrively drawing your attetion to
certain parts of the sentence we create a gap that our psychology is programmed to fill. So, we
need less and can run times faster.
    """

test_string = """
tldr - Simplified and community-driven man pages

Usage: tldr [-v|--version]
            ((-u|--update) | [-p|--platform PLATFORM] COMMAND | (-a|--about))
  tldr Client program

Available options:
  -h,--help                Show this help text
  -v,--version             Show version
  -u,--update              Update offline cache of tldr pages
  -p,--platform PLATFORM   Prioritize specfic platform while searching. Valid
                           values include linux, osx, windows, sunos
  COMMAND                  name of the command
  -a,--about               About this program

Hey there! What is up? Pi is 3.23425 Welcome 34 235523 to the fast lane, my friend. 
See, by selectrively drawing your attetion to certain parts of the sentence we 
create a gap that our psychology is programmed to fill. So, we need less and 
can run times faster.
    """


print(parse_text(test_string))
