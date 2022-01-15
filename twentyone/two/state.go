package main

import "fmt"

type GameState struct {
	players [2]Player
	die     Die
	turn    int
}

func (g GameState) Play() [3]GameState {
	out := [3]GameState{}
	playing := g.players[g.turn]
	newTurn := 1 - g.turn

	rolls := g.die.RollN(3)

	for i := 0; i < 3; i++ {
		player := playing
		player.Play(rolls[i])

		bothPlayers := g.players
		bothPlayers[g.turn] = player

		newState := GameState{
			players: bothPlayers,
			die:     g.die,
			turn:    newTurn,
		}

		out[i] = newState
	}

	return out
}

func (g GameState) Universes() (int, int) {
	states := map[GameState]int{}
	states[g] = 1

	winners := [...]int{0, 0}

	for i := 0; ; i++ {
		fmt.Println("generation:", i, "universes:", len(states))
		allWinning := true

		nextGen := map[GameState]int{}

		for state, universes := range states {
			// fmt.Println("turn:", state.turn)

			won, winner := state.Won()
			if won {
				winners[winner] += universes
				continue
			}

			allWinning = false

			splits := state.Play()
			for _, split := range splits {
				nextGen[split] += universes
			}
		}

		states = nextGen

		if allWinning {
			break
		}
	}

	return winners[0], winners[1]
}

func (g GameState) Won() (bool, int) {
	for i, player := range g.players {
		if player.Won() {
			return true, i
		}
	}

	return false, 0
}

func NewState(input string) GameState {
	one, two := parsePlayers(input)
	players := [...]Player{one, two}

	die := newDie()

	return GameState{
		players: players,
		die:     die,
		turn:    0,
	}
}
