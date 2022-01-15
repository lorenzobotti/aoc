package main

import (
	"errors"
	"regexp"
	"strconv"
	"strings"
)

type Player struct {
	Position int

	Score int
}

func (p *Player) Move(die *Die) {
	rolls := die.RollN(3)
	moveBy := 0
	for _, roll := range rolls {
		moveBy += roll
	}

	destination := (p.Position + moveBy) % 10
	p.Position = destination
	p.Score += destination + 1
}

func (p Player) Won() bool {
	return p.Score >= 1000
}

var playerRegex = regexp.MustCompile(`Player (\d*) starting position: (\d*)`)

func parsePlayer(line string) (Player, error) {
	res := playerRegex.FindStringSubmatch(line)
	if len(res) < 3 {
		return Player{}, errors.New("can't parse line")
	}

	playerNum := res[1]
	_ = playerNum
	positionRaw := res[2]
	position, err := strconv.Atoi(positionRaw)
	if err != nil {
		return Player{}, err
	}

	return Player{
		Position: position - 1,
		Score:    0,
	}, nil
}

func parsePlayers(input string) ([2]Player, error) {
	lines := strings.Split(input, "\n")
	players := [2]Player{}

	if len(lines) != 2 {
		return players, errors.New("wrong number of lines")
	}

	for i, line := range lines {
		player, err := parsePlayer(line)
		if err != nil {
			return players, err
		}

		players[i] = player
	}

	return players, nil
}
