package main

import (
	"regexp"
	"strconv"
	"strings"
)

type Player struct {
	score    int
	position int
}

func (p Player) Won() bool {
	return p.score >= 21
}

func (p *Player) Play(roll int) {
	p.position = (p.position + roll) % 10
	p.score += p.position
}

var playerRegex = regexp.MustCompile(`Player (\d*) starting position: (\d*)`)

func parsePlayer(line string) Player {
	res := playerRegex.FindStringSubmatch(line)

	pos, err := strconv.Atoi(res[2])
	if err != nil {
		panic(err)
	}

	return Player{
		position: pos - 1,
		score:    0,
	}
}

func parsePlayers(input string) (Player, Player) {
	lines := strings.Split(input, "\n")

	return parsePlayer(lines[0]), parsePlayer(lines[1])
}
