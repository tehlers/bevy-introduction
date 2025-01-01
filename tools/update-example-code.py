#! /usr/bin/env python3

import os
import re
import sys

INCLUDE_PATTERN = re.compile('<!-- include-code: (.*)§(\\d+) -->')

def load_example_code(filename, example_number):
    start_pattern = re.compile(' *// example-start: %s$| *// example-start: %s ({.*})' % (example_number, example_number))
    end_pattern = re.compile(' *// example-end: %s$' % example_number)

    content = []
    with open(filename, 'r') as file:
        include_content = False

        for line in file:
            if include_content:
                if line.find('// example-') == -1:
                    content.append(line)
                include_content = not end_pattern.match(line)
            else:
                include_content = start_pattern.match(line)
                if include_content:
                    if include_content.group(1):
                        content.append('```rust +line_numbers %s\n' % include_content.group(1))
                    else:
                        content.append('```rust +line_numbers\n')

        content.append('```\n')
    return content


def update_example_code(filename):
    content = []

    with open(filename, 'r') as file:
        skip_until_end_of_code_block = False

        for line in file:
            if not skip_until_end_of_code_block:
                content.append(line)
            else:
                skip_until_end_of_code_block = line != '```\n'

            m = INCLUDE_PATTERN.match(line)
            if m:
                content.extend(load_example_code(os.path.join(os.path.dirname(os.path.realpath(filename)), m.group(1)), m.group(2)))
                skip_until_end_of_code_block = True

    return ''.join(content)

def main():
    if not os.path.isfile(sys.argv[1]):
        print("%s is not a file" % sys.argv[1])
        sys.exit(1)

    updated_markdown = update_example_code(sys.argv[1])
    print(updated_markdown, end='')

if __name__ == "__main__":
    main()
