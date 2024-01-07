def extract_numeric_value(s):
    return next((int(word) for word in s.split() if word.isdigit()), None)


def calc_first(line):
    index = line.find(":")
    if index != -1:
        game = line[index + 2:]
        sets = [s.strip() for s in game.split(';')]

        for set_str in sets:
            red_count, green_count, blue_count = 0, 0, 0
            items = [item.strip() for item in set_str.split(',')]
            for item in items:
                if "red" in item:
                    value = extract_numeric_value(item)
                    if value is not None:
                        red_count += value
                elif "green" in item:
                    value = extract_numeric_value(item)
                    if value is not None:
                        green_count += value
                elif "blue" in item:
                    value = extract_numeric_value(item)
                    if value is not None:
                        blue_count += value

            if red_count > 12 or green_count > 13 or blue_count > 14:
                return False
        return True
    else:
        return False


def calc_second(line):
    max_red_count, max_green_count, max_blue_count = 0, 0, 0

    index = line.find(":")
    if index != -1:
        game = line[index + 2:]
        sets = [s.strip() for s in game.split(';')]

        for set_str in sets:
            red_count, green_count, blue_count = 0, 0, 0
            items = [item.strip() for item in set_str.split(',')]
            for item in items:
                if "red" in item:
                    value = extract_numeric_value(item)
                    if value is not None and red_count < value:
                        red_count = value
                elif "green" in item:
                    value = extract_numeric_value(item)
                    if value is not None and green_count < value:
                        green_count = value
                elif "blue" in item:
                    value = extract_numeric_value(item)
                    if value is not None and blue_count < value:
                        blue_count = value

            max_red_count = max(max_red_count, red_count)
            max_green_count = max(max_green_count, green_count)
            max_blue_count = max(max_blue_count, blue_count)

    return max_red_count * max_green_count * max_blue_count


def main():
    path = 'day02.txt'
    first_sum, second_sum = 0, 0
    with open(path, 'r') as file:
        for idx, line in enumerate(file):
            line = line.strip()
            if calc_first(line):
                first_sum += idx + 1
            second_sum += calc_second(line)
    print(f"Part 1: {first_sum}")
    print(f"Part 2: {second_sum}")


if __name__ == '__main__':
    main()
