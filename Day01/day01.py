def calc_first(line):
    first_digit = next((int(c) for c in line if c.isdigit()), 0)
    last_digit = next((int(c) for c in reversed(line) if c.isdigit()), 0)
    return first_digit * 10 + last_digit


def calc_second(line):
    digit_map = {
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
    }

    first_number, last_number = 0, 0
    first_number_found = False
    current_number = ""

    for i, c in enumerate(line):
        for cc in line[i:]:
            current_number += cc
            if current_number in digit_map:
                if not first_number_found:
                    first_number_found = True
                    first_number = digit_map[current_number]
                last_number = digit_map[current_number]
        current_number = ""

        if c.isdigit():
            if not first_number_found:
                first_number_found = True
                first_number = int(c)
            last_number = int(c)
    return first_number * 10 + last_number


def main():
    path = 'day01.txt'
    first_sum, second_sum = 0, 0
    with open(path, 'r') as file:
        for line in file:
            first_sum += calc_first(line)
            second_sum += calc_second(line)
    print(f"Part 1: {first_sum}")
    print(f"Part 2: {second_sum}")


if __name__ == '__main__':
    main()
