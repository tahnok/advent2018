import re
import sys
import unittest


def marble_winner(num_players, last_marble):
    ring = [0]
    current_marble = 0
    current_player = 0
    scores = {}
    for marble in range(1, last_marble + 1):
        current_player = (current_player + 1) % num_players

        if marble % 23 != 0:
            current_marble = (current_marble + 1) % len(ring)
            ring.insert(current_marble + 1, marble)
            current_marble = (current_marble + 1) % len(ring)
        else:
           score = scores.get(current_player, 0)
           score = score + marble

           to_remove = (current_marble - 7) % len(ring)
           score = score + ring.pop(to_remove)
           current_marble = to_remove

           scores[current_player] = score
        # print(f"{ring=} {scores=} {marble=} {current_marble=}")

    return max(scores.values())


class TestMarbleWinner(unittest.TestCase):
    def test_marble_winner(self):
        tests = [
            (9, 25, 32),
            (10, 1618, 8317),
            (13, 7999, 146373),
            (17, 1104, 2764),
            (21, 6111, 54718),
            (30, 5807, 37305)
        ]
        for (players, marbles, expected_score) in tests:
            self.assertEqual(marble_winner(players, marbles), expected_score)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("please pass the input as an argument to this script")
        sys.exit(-1)
    argument = sys.argv[1]

    with open(argument) as f:
        lines = f.readlines()
    line = lines[0]

    matches = re.search(
        r'(\d+) players; last marble is worth (\d+) points',
        line)
    num_players = int(matches.group(1))
    last_marble = int(matches.group(2))

    print(marble_winner(num_players, last_marble))
